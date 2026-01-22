package cmd

import (
	"flag"
	"fmt"
	"net"
	"os"
	"strconv"

	"github.com/anhtr13/tcpchat/tcpchat-client/internal"
)

var (
	HOST string
	PORT int
)

func init() {
	default_host := os.Getenv("HOST")
	if default_host == "" {
		default_host = "localhost"
	}

	default_port := 8080
	if port, err := strconv.ParseInt(os.Getenv("PORT"), 10, 32); err == nil {
		default_port = int(port)
	}

	flag.StringVar(&HOST, "host", default_host, "Specify server host")
	flag.IntVar(&PORT, "port", default_port, "Specify port number")
	flag.Parse()
}

func Execute() {
	conn, err := net.Dial("tcp", net.JoinHostPort(HOST, fmt.Sprintf("%d", PORT)))
	if err != nil {
		fmt.Println(err)
		return
	}
	defer conn.Close()

	fmt.Printf("Connected to %s, port %d.\n", HOST, PORT)

	go internal.HandleServerMessages(conn)
	internal.HandleInput(conn)
}
