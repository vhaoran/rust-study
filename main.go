package main 

import (
	"fmt"
	"net/http"
)

func sayhelloName(w http.ResponseWriter, r *http.Request){
	fmt.Fprintf(w, "hello world,go web!!")
}

func main() {
	http.HandleFunc("/goweb.go", sayhelloName)
        fmt.Println("listen at: 9999")
	http.ListenAndServe(":9999", nil)
}


