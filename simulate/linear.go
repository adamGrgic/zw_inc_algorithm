package simulate

import (
	"math/rand"
)

type LinearResult struct {
	Index int
	M     float64
	X     int
	B     float64
	Y     float64
}

func RunLinearSimulation(index int) LinearResult {
	m := rand.Float64()*1.5 + 0.5
	x := rand.Intn(101)
	b := rand.Float64() * 50
	y := m*float64(x) + b

	return LinearResult{
		Index: index,
		M:     m,
		X:     x,
		B:     b,
		Y:     y,
	}
}
