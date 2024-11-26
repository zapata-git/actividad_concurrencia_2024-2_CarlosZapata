package main

import (
	"fmt"
	"sync"
)

var (
	counter int
	mutex   sync.Mutex
)

func child(wg *sync.WaitGroup) {
	defer wg.Done()
	for i := 0; i < 10000000; i++ {
		mutex.Lock()
		counter++
		mutex.Unlock()
	}
}

func main() {
	var wg sync.WaitGroup

	wg.Add(2)
	go child(&wg)
	go child(&wg)

	wg.Wait()
	fmt.Printf("result: %d (should be 20000000)\n", counter)
}