package main

import (
	"fmt"
	"math"
	"sync"
)

type SimulationParams struct {
	Simulations  int
	Ticks        int
	InitialSlope float64
	SlopeDelta   float64
}

// func generateLinearData(simulations int, ticks int, initial_slope float64, slope_delta float64) {

// 	// ticks / simulation (ticks per simulation)

// 	// if I start with y = mx + b
// 	// m is slope
// 	// x is our variable

// 	// I want to graph each point (y)
// 	// M = slope
// 	// => could be a range of of numbers, along with standard deviations
// 	// => so given starting of -10 and ending of 10, get 10 evenly spread numbers
// 	// => add absolute value of both ends of range (10 + 10) and then divide that by our ticks
// 	// => 20 range points / 10 ticks = 2 increments per tick
// 	// B = point where y axis intercepts slope
// 	// -> for our purposes we can just keep this at 0 / don't need to parameterize
// 	// need the amount of 'ticks', or times we calculate y
// 	//

// 	// x is constant, m is variable
// 	// y = mx
// 	// ex: ticks: 10, m ranges from -10 to 10, x is 5
// 	// 1: y = -10 * 5 = -50
// 	// 2: y = -8 * 5 = -40
// 	// 3: y = -6 * 5 = -30
// 	// 4: y = -4 * 5 = -20
// 	// (ok so thats a pretty long range, maybe pick some smaller numbers to start)
// 	//

// 	// ...
// 	// given a range 1 to 5 (which means 5 simulations)
// 	// constant slope => 2
// 	// 10 ticks
// 	// 1: y = 1*1
// 	// 2: y = 1*2
// 	// 3: y = 1*3
// 	// 4: y = 1*4
// 	// 5: y = 1*5
// 	// ...

// 	count := 0

// 	results := make(map[int][]float64)

// 	for i := 1; i <= simulations; i++ {
// 		fmt.Println("Running simulation number: ", i)

// 		slope := initial_slope + slope_delta*float64(i)
// 		// y = mx + b
// 		var value_container []float64

// 		for j := range ticks {
// 			y := float64(j) * slope
// 			value_container = append(value_container, y)

// 		}
// 		results[i] = value_container
// 		count++
// 	}

// 	fmt.Println(results)
// 	// fmt.Println(data)
// }

func generateLinearData(sp SimulationParams) map[int][]float64 {
	var wg sync.WaitGroup
	resultsChan := make(chan struct {
		index int
		data  []float64
	}, sp.Simulations)

	for i := 1; i <= sp.Simulations; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()

			fmt.Println("Running simulation number:", i)

			slope := sp.InitialSlope + sp.SlopeDelta*float64(i)
			var value_container []float64

			for j := 0; j < sp.Ticks; j++ {
				y := math.Round(float64(j) * slope)
				value_container = append(value_container, y)
			}

			resultsChan <- struct {
				index int
				data  []float64
			}{index: i, data: value_container}
		}(i)
	}

	wg.Wait()
	close(resultsChan)

	// Collect results
	finalResults := make(map[int][]float64)
	for result := range resultsChan {
		finalResults[result.index] = result.data
	}

	fmt.Println(finalResults)
	return finalResults
}

func main() {

	generateLinearData(SimulationParams{Simulations: 10, Ticks: 10, InitialSlope: 2, SlopeDelta: 2})
	// 	data := [][]float64{
	// 		generateQuadraticData(1, -2, -3),   // Red
	// 		generateQuadraticData(0.5, 0, 1),   // Yellow
	// 		generateQuadraticData(-1, 2, 5),    // Green
	// 		generateQuadraticData(0.2, -1, 10), // Blue
	// 	}

	// graph := asciigraph.PlotMany(data,
	// 	asciigraph.Height(15),
	// 	asciigraph.Precision(1),
	// 	asciigraph.SeriesColors(
	// 		asciigraph.Red,
	// 		asciigraph.Yellow,
	// 		asciigraph.Green,
	// 		asciigraph.Blue,
	// 	),
	// 	asciigraph.Caption("Quadratic Curves"),
	// )

	// fmt.Println(graph)
}
