package utils

type Vec2 struct {
	X, Y int
}

type Pos2 struct {
	X, Y int
}

func (p *Pos2) Add(v Vec2) {
	p.X += v.X
	p.Y += v.Y
}

func (p *Pos2) Sub(that Pos2) {
	p.X -= that.X
	p.Y -= that.Y
}

func (this Pos2) Equals(that Pos2) bool {
	return this.X == that.X && this.Y == that.Y
}