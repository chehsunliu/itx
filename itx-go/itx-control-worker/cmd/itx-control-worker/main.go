package main

import (
	"log"

	contractqueue "github.com/chehsunliu/itx/itx-go/itx-contract/queue"
	"github.com/chehsunliu/itx/itx-go/itx-control-worker/internal/dispatcher"
	"github.com/chehsunliu/itx/itx-go/itx-control-worker/internal/state"
	"github.com/chehsunliu/itx/itx-go/itx-impl/worker"
)

func main() {
	s, err := state.FromEnv()
	if err != nil {
		log.Fatal(err)
	}
	queues := []contractqueue.MessageQueue{s.ControlStandardQueue, s.ControlPremiumQueue}
	worker.Run(queues, dispatcher.New(s))
}
