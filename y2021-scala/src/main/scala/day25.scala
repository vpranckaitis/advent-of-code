import java.lang.Math.max
import scala.annotation.tailrec
import scala.io.{Source, StdIn}

object day25 extends App:
  val input = Source.fromFile("files/25.in")

  val data = input.getLines().map { _.toIndexedSeq }.toIndexedSeq

  println(part1(data.toSeq))

  input.close()


  type Grid = IndexedSeq[IndexedSeq[Char]]


  def part1(grid: Grid): Int =
    move(1, grid)

  @tailrec
  def move(k: Int, grid: Grid): Int =
    val newGrid = moveSouth(moveEast(grid))
    if grid != newGrid then
      move(k + 1, newGrid)
    else
      k

  def moveEast(grid: Grid): Grid =
    val newPositions = for
      i <- grid.indices
      j <- grid.head.indices
      if grid(i)(j) == '>'
      jj = (j + 1) % grid.head.size
    yield if grid(i)(jj) == '.' then (i, jj) else (i, j)

    val newPositionsSet = newPositions.toSet

    grid.zipWithIndex.map { case (row, i) =>
      row.zipWithIndex.map { case (c, j) =>
        if c == 'v' then
          'v'
        else if newPositionsSet((i, j)) then
          '>'
        else
          '.'
      }
    }

  def moveSouth(grid: Grid): Grid =
    val newPositions = for
      i <- grid.indices
      j <- grid.head.indices
      if grid(i)(j) == 'v'
      ii = (i + 1) % grid.size
    yield if grid(ii)(j) == '.' then (ii, j) else (i, j)

    val newPositionsSet = newPositions.toSet

    grid.zipWithIndex.map { case (row, i) =>
      row.zipWithIndex.map { case (c, j) =>
        if c == '>' then
          '>'
        else if newPositionsSet((i, j)) then
          'v'
        else
          '.'
      }
    }

