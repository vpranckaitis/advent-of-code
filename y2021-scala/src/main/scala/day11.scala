import java.lang.Integer.parseInt
import java.lang.Math.abs
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.io.{Source, StdIn}

object day11 extends App:
  val Size = 10
  val MaxEnergy = 9

  val input = Source.fromFile("files/11.in")

  val data = input.getLines().map { _.split("").map(parseInt).toSeq }.toSeq

  println(part1(data))
  println(part2(data))

  input.close()


  @tailrec
  def makeEnergyIncreases(q: Queue[(Int, Int)], grid: Seq[Seq[Int]], inc: Map[(Int, Int), Int]): Map[(Int, Int), Int] =
    q match
      case (i, j) +: rest if grid(i)(j) + inc.getOrElse((i, j), 0) == MaxEnergy =>
        val neighbors = for
          di <- -1 to 1
          dj <- -1 to 1
          if abs(di) + abs(dj) != 0
          ii = i + di
          jj = j + dj
          if ii >= 0 && ii < Size && jj >= 0 && jj < Size
        yield (ii, jj)

        val updatedInc = inc.updatedWith((i, j)) {
          case Some(v) => Some(v + 1)
          case None => Some(1)
        }

        makeEnergyIncreases(rest.appendedAll(neighbors), grid, updatedInc)
      case (i, j) +: rest =>
        val updatedInc = inc.updatedWith((i, j)) {
          case Some(v) => Some(v + 1)
          case None => Some(1)
        }
        makeEnergyIncreases(rest, grid, updatedInc)
      case _ => inc


  @tailrec
  def repeat(k: Int, grid: Seq[Seq[Int]], flashes: Int, repeatUntil: (Int, Seq[Seq[Int]]) => Boolean): (Int, Int) =
    if repeatUntil(k, grid) then
      (k, flashes)
    else
      val q = for
        i <- 0 until Size
        j <- 0 until Size
      yield (i, j)

      val increases = makeEnergyIncreases(Queue.from(q), grid, Map.empty)

      val updatedGrid: Seq[Seq[Int]] = grid.zipWithIndex.map { (row, i) =>
        row.zipWithIndex.map { (c, j) =>
          val updated: Int = c + increases.getOrElse((i, j), 0)
          if updated <= MaxEnergy then
            updated
          else
            0
        }
      }

      val updatedFlashes = flashes + updatedGrid.map { _.count { _ == 0 } }.sum

      repeat(k + 1, updatedGrid, updatedFlashes, repeatUntil)


  def part1(data: Seq[Seq[Int]]): Int =
    repeat(0, data, 0, { (k, _) => k == 100 })._2


  def part2(data: Seq[Seq[Int]]): Int =
    repeat(0, data, 0, { (_, grid) =>
      grid.forall { _.forall { _ == 0 } }
    })._1



