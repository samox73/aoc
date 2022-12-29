package utils

type Vec2 struct {
	x, y int
}

type Pos2 struct {
	x, y int
}

func (p *Pos2) Add(v Vec2) {
	p.x += v.x
	p.y += v.y
}

func (p *Pos2) Sub(that Pos2) {
	p.x -= that.x
	p.y -= that.y
}

func (this Pos2) Equals(that Pos2) bool {
	return this.x == that.x && this.y == that.y
}