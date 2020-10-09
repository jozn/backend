
package main

import (
	"fmt"

	ghasedak "github.com/ghasedakapi/ghasedak-go"
)

func main() {
	c := ghasedak.NewClient("57d606af03970b8713840cdef028fff46fe2f677243a8547d1a3ebfbe4c3ab23", "300002525")

	r := c.Send("Hello, Brave new world!", "09015132328")
	fmt.Println(r.Code)
	fmt.Println(r.Message)
}