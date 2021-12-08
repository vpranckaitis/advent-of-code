import java.lang.Integer.parseInt
import java.lang.Math.{max, min}
import scala.annotation.tailrec
import scala.io.{Source, StdIn}

object day05 extends App:
  case class Point(x: Int, y: Int)

  val input = Source.fromFile("files/05.in")

  val directions = input.getLines().map {
    case s"$x1,$y1 -> $x2,$y2" => (Point(parseInt(x1), parseInt(y1)), Point(parseInt(x2), parseInt(y2)))
  }.toSeq

  println(part1(directions))

  println(part2(directions))

  input.close()


  def part1(directions: Seq[(Point, Point)]): Int =
    val lines = directions.filter { case (p1, p2) => p1.x == p2.x || p1.y == p2.y }

    val pos = lines.flatMap { case (p1, p2) =>
      for {
        x <- min(p1.x, p2.x) to max(p1.x, p2.x)
        y <- min(p1.y, p2.y) to max(p1.y, p2.y)
      } yield (x, y)
    }

    pos.groupBy(identity).count { case (_, v) => v.sizeIs > 1 }


  def part2(directions: Seq[(Point, Point)]): Int =
    val (lines, diagonals) = directions.partition { case (p1, p2) => p1.x == p2.x || p1.y == p2.y }

    val posLines = lines.flatMap { case (p1, p2) =>
      for {
        x <- min(p1.x, p2.x) to max(p1.x, p2.x)
        y <- min(p1.y, p2.y) to max(p1.y, p2.y)
      } yield (x, y)
    }

    val posDiagonals = diagonals.flatMap { case (p1, p2) =>
      val (pFrom, pTo) = if p1.x < p2.x then (p1, p2) else (p2, p1)
      val length = pTo.x - pFrom.x
      val yDir = if pFrom.y < pTo.y then 1 else -1
      for (i <- 0 to length) yield (pFrom.x + i, pFrom.y + i * yDir)
    }

    (posLines ++ posDiagonals).groupBy(identity).count { case (_, v) => v.sizeIs > 1 }