package dispatcher

import (
	"context"
	"log/slog"

	"github.com/chehsunliu/itx/itx-go/itx-compute-worker/internal/state"
)

type Dispatcher struct {
	//nolint:unused // wired up by upcoming handlers
	state state.WorkerState
}

func New(state state.WorkerState) *Dispatcher {
	return &Dispatcher{state: state}
}

func (d *Dispatcher) Handle(_ context.Context, body string) error {
	slog.Info("compute message received (no handler yet)", "body", body)
	return nil
}
