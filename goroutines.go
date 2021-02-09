package main

import (
	"fmt"
	"os"
	"sync"
	"time"
)

func compute() {
	wg := sync.WaitGroup{}
	wg.Add(1000)
	for i := 0; i < 1000; i++ {
		go func() {
			defer wg.Done()
			buffer := make([]byte, 10)
			devUrandom, _ := os.Open("/dev/urandom")
			devUrandom.Read(buffer)
			devNull, _ := os.OpenFile("/dev/null", os.O_APPEND|os.O_WRONLY, 0644)
			devNull.Write(buffer)
		}()
	}
	wg.Wait()
}

func main() {
	// warmup
	compute()

	before := time.Now()
	for i := 0; i < 1000; i++ {
		compute()
	}
	elapsed := time.Now().Sub(before)
	fmt.Println(elapsed, "total,", elapsed/1000, "avg per iteration")
}
