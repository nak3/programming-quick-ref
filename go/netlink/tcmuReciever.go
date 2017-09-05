package main

import (
	"fmt"
	"log"

	"github.com/mdlayher/genetlink"
	"github.com/mdlayher/netlink"
)

func main() {
	tcmu_netlink()
}

func tcmu_netlink() {
	log.Printf("starting...")
	c, err := genetlink.Dial(nil)
	if err != nil {
		log.Fatalf("failed to dial netlink: %v", err)
	}
	defer c.Close()

	family, err := c.GetFamily("TCM-USER")
	if err != nil {
		//TODO
	}
	var groupID uint32
	for _, g := range family.Groups {
		if g.Name == "config" {
			groupID = family.Groups[0].ID
			break
		}
	}
	if groupID == 0 {
		//TODO This must be not necessary as GetFamily already worked.
		log.Fatalf("not found")
	}
	c.JoinGroup(groupID)

	// Perform a request, receive replies, and validate the replies
	for {
		log.Printf("receiving...")
		msgs, _, err := c.Receive()
		if err != nil {
			log.Fatalf("failed to execute request: %v", err)
		}
		fmt.Printf(" %#v \n", msgs)
		atbs, _ := netlink.UnmarshalAttributes(msgs[0].Data)
		fmt.Printf(" %#v \n", atbs)
		for i, _ := range atbs {
			fmt.Printf(" %s \n", atbs[i].Data)
		}
	}
}
