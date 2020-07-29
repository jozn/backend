package main

import (
	"fmt"
	"os"
	"runtime"
	"sync"
)

func main5() {

    _  = make([]*int, 15e8)
    m := make(map[int]*string)


    for j:=0; j < 100; j++ {
        for i:=0; i < 50_000_000; i++ {
            go func () {
                s:= "s"
                _ = &s
            }()
            s:= "s"
            m[i] = &s
           if i%1_000_000 == 0 {
                fmt.Println(j ,i,&s)
           }
        }
    }

}

func main3() {
    for i:=0; i < 1000_000_000; i++ {

        s:= "s"
       if i%10_000_000 == 0 {
            fmt.Println("s",i,&s)
       }
    }

}

func main() {
	// A huge allocation to give.. the GC work to do
	lotsOf := make([]*int, 1e8)
	fmt.Println("Background GC work generated")
	// Force a GC to set a baseline we can see if we set GODEBUG=gctrace=1
	runtime.GC()

	// Use up all the CPU doing work that causes allocations that could be cleaned up by the GC.
	var wg sync.WaitGroup
	numWorkers := runtime.NumCPU()
	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			work()
		}()
	}

	wg.Wait()

	// Make sure that this memory isn't optimised away
	runtime.KeepAlive(lotsOf)
}

func work() {
	for {
		work := make([]*int, 1e6)
		if f := factorial(20); f != 2432902008176640000 {
			fmt.Println(f)
			os.Exit(1)
		}
		runtime.KeepAlive(work)
	}
}

func factorial(n int) int {
	if n == 1 {
		return 1
	}
	return n * factorial(n-1)
}