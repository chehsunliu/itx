package main

import (
	"log"

	"github.com/chehsunliu/itx/itx-go/itx-compute-worker/internal/dispatcher"
	"github.com/chehsunliu/itx/itx-go/itx-compute-worker/internal/state"
	contractqueue "github.com/chehsunliu/itx/itx-go/itx-contract/queue"
	"github.com/chehsunliu/itx/itx-go/itx-impl/worker"
)

func main() {
	s, err := state.FromEnv()
	if err != nil {
		log.Fatal(err)
	}
	queues := []contractqueue.MessageQueue{s.ComputeStandardQueue, s.ComputePremiumQueue}
	worker.Run(queues, dispatcher.New(s))
}
