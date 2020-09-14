package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"time"
)

func getFrame(address string) (response map[string]interface{}, err error) {
	url := fmt.Sprintf("http://%s:6721/session", address)
	log.Println(url)

	resp, err := http.Get(url)
	if err != nil {
		log.Printf("HTTP error: %v", err)
		return
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)

	response = make(map[string]interface{})
	err = json.Unmarshal(body, &response)
	if err != nil {
		log.Printf("Response could not be decoded as JSON:\n%v", err)
		return
	}

	return
}

func main() {
	address := flag.String("address", "127.0.0.1", "IP address where Echo VR can be reached; if using Quest, this is your Quests's WiFi IP address")
	flag.Parse()

	for {
		response, err := getFrame(*address)
		if err != nil {
			log.Printf("Could not access API, trying again in 3 seconds")
			time.Sleep(3 * time.Second)
			continue
		}

		log.Printf("%v", response)
	}
}
