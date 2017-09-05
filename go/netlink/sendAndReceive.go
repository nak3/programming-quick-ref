package main

import (
	"fmt"
	"log"
	"os"
	"time"

	"github.com/mdlayher/genetlink"
	"github.com/mdlayher/netlink"
)

func main() {
	ExampleConn_nl80211WiFi()
}

func ExampleConn_nl80211WiFi() {
	c, err := genetlink.Dial(nil)
	if err != nil {
		log.Fatalf("failed to dial generic netlink: %v", err)
	}
	defer c.Close()

	// Constants which are sourced from nl80211.h.
	const (
		name                       = "nl80211"
		nl80211CommandGetInterface = 5
	)

	go func() {
		for {
			msgs, _, err := c.Receive()
			if err != nil {
				log.Fatalf("failed to execute request: %v", err)
			}
			fmt.Printf("received: %#v \n", msgs)
		}
	}()
	time.Sleep(1 * time.Second)

	// Ask generic netlink if nl80211 is available
	family, err := c.GetFamily("nl80211")
	if err != nil {
		if os.IsNotExist(err) {
			log.Printf("%q family not available", name)
			return
		}
		log.Fatalf("failed to query for family: %v", err)
	}

	// Ask nl80211 to dump a list of all WiFi interfaces
	req := genetlink.Message{
		Header: genetlink.Header{
			Command: nl80211CommandGetInterface,
			Version: family.Version,
		},
	}
	flags := netlink.HeaderFlagsRequest | netlink.HeaderFlagsDump
	msgs, err := c.Send(req, family.ID, flags)
	if err != nil {
		log.Fatalf("failed to execute: %v", err)
	}
	// Some basic information about a WiFi interface
	time.Sleep(2 * time.Second)
	fmt.Printf("sent: %v", msgs)
}
