package main

import (
	"bytes"
	"fmt"
	"image/png"
	"io/ioutil"
	"time"
)

func main() {
	for i := 0; i < 10; i++ {
		startTime := time.Now()

		data, err := ioutil.ReadFile("rust.png")
		if err != nil {
			panic(err)
		}
		rd := bytes.NewReader(data)
		_, err = png.Decode(rd)
		if err != nil {
			panic(err)
		}

		fmt.Println("耗时：", time.Now().Sub(startTime))
	}
}
