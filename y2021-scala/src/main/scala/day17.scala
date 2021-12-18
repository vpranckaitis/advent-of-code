import java.lang.Integer.parseInt
import java.lang.Math.{abs, max}
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.collection.mutable
import scala.io.{Source, StdIn}
import scala.util.{Success, Try}

object day17 extends App:
  val input = Source.fromFile("files/17.in")

  val s"target area: x=${ParseInt(x1)}..${ParseInt(x2)}, y=${ParseInt(y1)}..${ParseInt(y2)}" = input.getLines().next()

  println(part1(x1, x2, y1, y2))
  println(part2(x1, x2, y1, y2))

  input.close()


  def part1(x1: Int, x2: Int, y1: Int, y2: Int): Int =
    val maxYs = for
      vx <- 0 to x2 + 1
      vy <- y1 - 1 to 1000
      maxY <- simulate(0, 0, vx, vy, x1, x2, y1, y2, 0)
    yield maxY

    maxYs.max


  def part2(x1: Int, x2: Int, y1: Int, y2: Int): Int =
    val maxYs = for
      vx <- 0 to x2 + 1
      vy <- y1 - 1 to 1000
      maxY <- simulate(0, 0, vx, vy, x1, x2, y1, y2, 0)
    yield maxY

    maxYs.length


  @tailrec
  def simulate(x: Int, y: Int, vx: Int, vy: Int, x1: Int, x2: Int, y1: Int, y2: Int, maxY: Int): Option[Int] =
    val updatedMaxY = max(y, maxY)
    if x >= x1 && x <= x2 && y >= y1 && y <= y2 then
      Some(updatedMaxY)
    else if y < y1 then
      None
    else if x > x2 then
      None
    else if vx == 0 && x < x1 then
      None
    else
      simulate(x + vx, y + vy, max(vx - 1, 0), vy - 1, x1, x2, y1, y2, updatedMaxY)


  object ParseInt:
    def unapply(s: String): Option[Int] = Try { Integer.parseInt(s) }.toOption