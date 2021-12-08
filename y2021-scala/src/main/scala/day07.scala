import java.lang.Integer.parseInt
import java.lang.Math.{abs, max, min}
import scala.annotation.tailrec
import scala.io.{Source, StdIn}

object day07 extends App:
  val input = Source.fromFile("files/07.in")

  val positions = input.getLines().next().split(",").map(parseInt)

  println(part1(positions))

  println(part2(positions))

  input.close()


  def part1(positions: Seq[Int]): Int =
    (positions.min to positions.max).map { p =>
      positions.map { q => abs(p - q) }.sum
    }.min


  def part2(positions: Seq[Int]): Long =
    def fuelCost(n: Long): Long =
      if n == 0 then 0 else ((1 + n) * n) / 2

    (positions.min to positions.max).map { p =>
      positions.map { q => fuelCost(abs(p - q).toLong) }.sum
    }.min
