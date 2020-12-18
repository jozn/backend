package main

import (
    "fmt"
    "net/http"
    "time"
)

func main() {
    http.HandleFunc("/", HelloServer)
    http.ListenAndServe(":8082", nil)
    time.Sleep(1000*time.Second)
    fmt.Println("end")
}

func HelloServer(w http.ResponseWriter, r *http.Request) {
//     fmt.Fprintf(w, "Hello, %s!", r.URL.Path[1:])
    fmt.Fprintf(w, "Hello, worls")
}