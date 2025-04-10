package simulate

import (
	"math"
	"math/rand"
)

type QuadraticResult struct {
	Index int
	A     float64
	B     float64
	C     float64
	X     float64
	Y     float64
}

func RunQuadraticSimulation(index int) QuadraticResult {
	a := rand.Float64()*2 + 1  // between 1 and 3
	b := rand.Float64()*5 + 1  // between 1 and 6
	c := rand.Float64()*10 + 1 // between 1 and 11
	x := rand.Float64() * 100

	y := a*math.Pow(x, 2) + b*x + c

	return QuadraticResult{
		Index: index,
		A:     a,
		B:     b,
		C:     c,
		X:     x,
		Y:     y,
	}
}
