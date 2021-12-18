import java.lang.Integer.parseInt
import java.lang.Math.abs
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.collection.mutable
import scala.io.{Source, StdIn}
import scala.util.{Success, Try}

object day15 extends App:
  val input = Source.fromFile("files/15.in")

  val grid = input.getLines().map { _.split("").map(parseInt).toSeq }.toSeq

  println(part1(grid))
  println(part2(grid))

  input.close()


  def part1(grid: Seq[Seq[Int]]): Int =
    val n = grid.length
    val m = grid.last.length
    val q = mutable.PriorityQueue.from(Seq((0, 0, 0)))(Ordering.Tuple3[Int, Int, Int].reverse)
    val distances = Map((0, 0) -> 0)
    shortestPath((n - 1, m - 1), q, distances, grid)


  def part2(grid: Seq[Seq[Int]]): Int =
    def wrap(k: Int) = if k > 9 then k - 9 else k

    val expandedHorizontally = grid.map { row =>
      (0 until 5).flatMap { k => row.map { v => wrap(v + k) } }
    }
    val expanded = (0 until 5).flatMap { k => expandedHorizontally.map { _.map { v => wrap(v + k) }} }
    part1(expanded)


  @tailrec
  def shortestPath(target: (Int, Int),
                   q: mutable.PriorityQueue[(Int, Int, Int)],
                   distances: Map[(Int, Int), Int],
                   grid: Seq[Seq[Int]]): Int =
    q.headOption match
      case None =>
        -1
      case Some((d, i, j)) if (i, j) == target =>
        d
      case Some((d, i, j)) if distances.get((i, j)).exists { _ < d } =>
        q.dequeue()
        shortestPath(target, q, distances, grid)
      case Some((d, i, j)) =>
        val n = grid.length
        val m = grid.last.length

        val neighbors = for
          di <- -1 to 1
          dj <- -1 to 1
          if abs(di) + abs(dj) == 1
          ii = i + di
          jj = j + dj
          if ii >= 0 && ii < n && jj >= 0 && jj < m
          dist = d + grid(ii)(jj)
          if !distances.get((ii, jj)).exists { _ <= dist }
        yield ((ii, jj), dist)

        val updatedDistances = distances ++ neighbors
        q.dequeue()
        q.enqueue(neighbors.map { case ((i, j), d) => (d, i, j)  }: _*)

        shortestPath(target, q, updatedDistances, grid)



