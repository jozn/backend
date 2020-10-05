package main

import (
	"fmt"
	"os"
	"runtime"
	"sync"
	"time"
)

func f() {
	s := "s532534523gsdfgsdfgsdgdgdsfgsdgdsgdsggdgdgsdgdgdfgdgdfgdf4523452345645645"
	_ = &s
	time.Sleep(time.Millisecond * 200)
	go f()
}

func heaping(j int) {
	m := make(map[int]*string)
	max := 80_000_000
	for i := 0; i < max; i++ {
		s := "mkflksdajfaldskjdlfkjds"
		m[i+max] = &s
		if i%10_000_000 == 0 {
			fmt.Println("m", j, i, &s)
		}
	}
	time.Sleep(time.Second * 10000)
}

var mp1 = make(map[int]*string)
var mp2 = make(map[int]*string)
var mp3 = make(map[int]*string)
var mp4 = make(map[int]*string)

var max = 50_000_000

func heap1() {
	for i := 0; i < max; i++ {
		s := "v"
		mp1[i] = &s
		if i%10_000_000 == 0 {
			fmt.Println("1", i, &s)
		}
	}
	go heap1Del()
}

func heap1Del() {
	for i := 0; i < max; i++ {
		s := "v"
		delete(mp1, i)
		if i%10_000_000 == 0 {
			fmt.Println("1 del", i, &s)
		}
	}
	go heap1()
}

func heap2() {
	for i := 0; i < max; i++ {
		s := "v"
		mp2[i] = &s
		if i%10_000_000 == 0 {
			fmt.Println("2", i, &s)
		}
	}
	fmt.Println("end")
	go heap2Del()
}

func heap2Del() {
	for i := 0; i < max; i++ {
		s := "v"
		delete(mp2, i)
		if i%10_000_000 == 0 {
			fmt.Println("1 del", i, &s)
		}
	}
	go heap2()
}
func heap3() {
	for i := 0; i < max; i++ {
		s := "v"
		mp3[i] = &s
		if i%10_000_000 == 0 {
			fmt.Println("3", i, &s)
		}
	}
	fmt.Println("end")

}

func heap4() {
	for i := 0; i < max; i++ {
		s := "v"
		mp4[i] = &s
		if i%10_000_000 == 0 {
			fmt.Println("4", i, &s)
		}
	}
	fmt.Println("end 4")
	for i := 0; i < 10000; i++ {
		go f()
	}
}

func main() {
	go heap1()
	go heap2()
	go heap3()
	go heap4()

	for i := 0; i < 1000; i++ {
		go f()
	}

	time.Sleep(time.Second * 10000000)

	fmt.Println(mp1[589789])
	fmt.Println(mp2[589789])
	fmt.Println(mp3[589789])
	fmt.Println(mp4[589789])

}

func main_old() {

	for i := 1; i < 5; i++ {
		go heaping(i)
	}

	for i := 0; i < 5000; i++ {
		go f()
	}

	time.Sleep(time.Second * 10000000)

}

func main3() {
	for i := 0; i < 1000_000_000; i++ {

		s := "s"
		if i%10_000_000 == 0 {
			fmt.Println("s", i, &s)
		}
	}

}

func main6() {
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
