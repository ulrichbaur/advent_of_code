// AoC 2024 Day 09: Disk Fragmenter
package main

import (
	"aoc/utils"
	"fmt"
)

type fileEntity struct {
	position int
	size     int
}

type spaceEntity struct {
	position int
	size     int
	used     int
}

type diskFile struct {
	id    int
	file  fileEntity
	space spaceEntity
}

func convertASCIINumeralToInt(b byte) int {
	return int(b - 48)
}

func parseDisk(line []byte) ([]diskFile, []int) {
	diskFiles := make([]diskFile, 0)
	disk := make([]int, 0)

	id := 0
	offset := 0
	for i := 0; i < len(line)-1; i += 2 {
		fileSize := convertASCIINumeralToInt(line[i])
		spaceSize := convertASCIINumeralToInt(line[i+1])

		diskFiles = append(diskFiles, diskFile{id, fileEntity{offset, fileSize}, spaceEntity{offset + fileSize, spaceSize, 0}})

		for range fileSize {
			disk = append(disk, id)
		}
		for range spaceSize {
			disk = append(disk, -1)
		}

		offset += fileSize + spaceSize
		id++
	}
	fileSize := convertASCIINumeralToInt(line[len(line)-1])
	diskFiles = append(diskFiles, diskFile{id, fileEntity{offset, fileSize}, spaceEntity{offset + fileSize, 0, 0}})
	for range fileSize {
		disk = append(disk, id)
	}

	return diskFiles, disk
}

func defragmentDiskPart1(diskInfo []diskFile, disk []int) []int {
	diskLength := len(disk) - 1
	for fileToMove := len(diskInfo) - 1; fileToMove > 0; fileToMove-- {
		remainingElementsToMove := diskInfo[fileToMove].file.size

		for candidate := 0; candidate < fileToMove; candidate++ {
			availableSpace := diskInfo[candidate].space.size - diskInfo[candidate].space.used
			if availableSpace > 0 {
				marked := 0
				for i := range int(diskInfo[candidate].space.size) {
					if disk[diskInfo[candidate].space.position+i] == -1 {
						disk[diskInfo[candidate].space.position+i] = diskInfo[fileToMove].id
						disk[diskLength-(diskInfo[fileToMove].file.size-remainingElementsToMove)-diskInfo[fileToMove].space.size] = -2
						marked++
						remainingElementsToMove--

						if remainingElementsToMove == 0 {
							break
						}
					}
				}
				diskInfo[candidate].space.used += marked
				if remainingElementsToMove == 0 {
					break
				}
			}
		}
		diskLength -= diskInfo[fileToMove].file.size + diskInfo[fileToMove].space.size
	}
	return disk
}

func defragmentDiskPart2(diskInfo []diskFile, disk []int) []int {
	diskLength := len(disk) - 1
	for fileToMove := len(diskInfo) - 1; fileToMove > 0; fileToMove-- {
		for candidate := 0; candidate < fileToMove; candidate++ {
			availableSpace := diskInfo[candidate].space.size - diskInfo[candidate].space.used
			if availableSpace >= diskInfo[fileToMove].file.size {
				marked := 0
				for i := range int(diskInfo[candidate].space.size) {
					if disk[diskInfo[candidate].space.position+i] == -1 {
						disk[diskInfo[candidate].space.position+i] = diskInfo[fileToMove].id
						disk[diskLength-marked-diskInfo[fileToMove].space.size] = -2
						marked++

						if marked == diskInfo[fileToMove].file.size {
							break
						}
					}
				}
				diskInfo[candidate].space.used += diskInfo[fileToMove].file.size
				break
			}
		}
		diskLength -= diskInfo[fileToMove].file.size + diskInfo[fileToMove].space.size
	}
	return disk
}

func calculateChecksum(nums []int) int {
	checksum := 0

	for i, num := range nums {
		if num > 0 {
			checksum += i * num
		}
	}

	return checksum
}

func solvePart1(lines []string) int {
	defer utils.Timer("day09p1")()

	diskInfo, disk := parseDisk([]byte(lines[0]))

	defragmented := defragmentDiskPart1(diskInfo, disk)

	checksum := calculateChecksum(defragmented)
	return checksum
}

func solvePart2(lines []string) int {
	defer utils.Timer("day09p2")()

	diskInfo, disk := parseDisk([]byte(lines[0]))

	defragmented := defragmentDiskPart2(diskInfo, disk)

	checksum := calculateChecksum(defragmented)
	return checksum
}

func main() {
	fmt.Println("AoC 2024 - Day 09: Disk Fragmenter")
	fmt.Println("==================================")

	lines, err := utils.ReadLines("day09/day09_input.txt")

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}
	// utils.PrintRows(lines)

	totalPart1 := solvePart1(lines)
	fmt.Println("File Checksum [split files] (Part 1 Solution):", totalPart1)

	totalPart2 := solvePart2(lines)
	fmt.Println("File Checksum [whole files] (Part 2 Solution):", totalPart2)
}
