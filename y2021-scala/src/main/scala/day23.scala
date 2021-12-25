import java.lang.Integer.parseInt
import java.lang.Math.{abs, max}
import scala.annotation.tailrec
import scala.collection.mutable
import scala.io.{Source, StdIn}
import scala.math.Ordering.Implicits.seqOrdering
import scala.util.{Success, Try}

object day23 extends App:
  val input = Source.fromFile("files/23.in")

  val lines = input.getLines()
  lines.next()
  lines.next()
  val row1 = lines.next().filter { _.isLetter }
  val row2 = "  #D#C#B#A#  ".filter { _.isLetter }
  val row3 = "  #D#B#A#C#  ".filter { _.isLetter }
  val row4 = lines.next().filter { _.isLetter }

  val pos1 = row1.zipWithIndex.map { case (c, i) => (c, i + 11) }
  val pos2 = row2.zipWithIndex.map { case (c, i) => (c, i + 15) }
  val pos3 = row3.zipWithIndex.map { case (c, i) => (c, i + 19) }
  val pos4 = row4.zipWithIndex.map { case (c, i) => (c, i + 23) }

  val pos = (pos1 ++ pos2 ++ pos3 ++ pos4).sorted.zipWithIndex.map { case ((_, u), _) => u.toByte }

  val costs = Map(
    0 -> 1,
    1 -> 1,
    2 -> 1,
    3 -> 1,
    4 -> 10,
    5 -> 10,
    6 -> 10,
    7 -> 10,
    8 -> 100,
    9 -> 100,
    10 -> 100,
    11 -> 100,
    12 -> 1000,
    13 -> 1000,
    14 -> 1000,
    15 -> 1000,
  )

  val graph = Map(
    0 -> Seq(1),
    1 -> Seq(0, 2),
    2 -> Seq(1, 3, 11),
    3 -> Seq(2, 4),
    4 -> Seq(3, 5, 12),
    5 -> Seq(4, 6),
    6 -> Seq(5, 7, 13),
    7 -> Seq(6, 8),
    8 -> Seq(7, 9, 14),
    9 -> Seq(8, 10),
    10 -> Seq(9),

    11 -> Seq(2, 15),
    12 -> Seq(4, 16),
    13 -> Seq(6, 17),
    14 -> Seq(8, 18),

    15 -> Seq(11, 19),
    16 -> Seq(12, 20),
    17 -> Seq(13, 21),
    18 -> Seq(14, 22),

    19 -> Seq(15, 23),
    20 -> Seq(16, 24),
    21 -> Seq(17, 25),
    22 -> Seq(18, 26),

    23 -> Seq(19),
    24 -> Seq(20),
    25 -> Seq(21),
    26 -> Seq(22),
  )

  val roomNumbers = Seq(
    Seq(11, 15, 19, 23),
    Seq(12, 16, 20, 24),
    Seq(13, 17, 21, 25),
    Seq(14, 18, 22, 26),
  )

  val cantStop = Set(2, 4, 6, 8)

  println(part2(pos, costs, graph))

  input.close()


  def part2(pos: Seq[Byte], costs: Map[Int, Int], graph: Map[Int, Seq[Int]]): Int =
    val q = mutable.PriorityQueue[(Int, Seq[Byte])]()(Ordering.Tuple2[Int, Seq[Byte]].reverse)
    q.enqueue((0, pos))
    sp(q, costs, graph, Map.empty)


  def isTargetRoom(idx: Int, u: Int): Boolean =
    idx / 4 == (u - 11) % 4


  def isRoom(u: Int) = u > 10


  def roomContainsStrangers(idx: Int, pos: Seq[Byte]): Boolean =
    !roomNumbers(idx/4).forall { r => !pos.contains(r) || pos.indexOf(r) / 4 == idx / 4 }


  @tailrec
  def sp(q: mutable.PriorityQueue[(Int, Seq[Byte])], costs: Map[Int, Int], graph: Map[Int, Seq[Int]], stateCosts: Map[Seq[Byte], Int]): Int =
    if q.isEmpty then
      -1
    else
      val (cost, pos) = q.dequeue()
      val finish = pos.zipWithIndex.forall { p => isTargetRoom(p._2, p._1) }
      if finish then
        cost
      else if stateCosts.getOrElse(pos, Integer.MAX_VALUE) < cost then
        sp(q, costs, graph, stateCosts)
      else
        val newStates = for
          idx <- pos.indices
          if (isRoom(pos(idx)) && (!isTargetRoom(idx, pos(idx)) || roomContainsStrangers(idx, pos))) ||
            (!isRoom(pos(idx)) && !roomContainsStrangers(idx, pos))
          (cost, newPos) <- dfs(idx, -1, pos, !isRoom(pos(idx)), cost, costs, graph)
          if newPos != pos
          if stateCosts.getOrElse(newPos, Integer.MAX_VALUE) > cost
        yield (cost, newPos)
        val updatedStateCosts = stateCosts ++ newStates.map { case (x, y) => (y, x) }
        q.enqueue(newStates: _*)
        sp(q, costs, graph, updatedStateCosts)


  def dfs(idx: Int, par: Int, pos: Seq[Byte], toRoom: Boolean, cost: Int, costs: Map[Int, Int], graph: Map[Int, Seq[Int]]): Seq[(Int, Seq[Byte])] =
    val u = pos(idx)
    val steps = for
      v <- graph(u)
      if v != par && !pos.contains(v)
      if (isRoom(u) == isRoom(v)) || (isRoom(v) == toRoom && (!isRoom(v) || isTargetRoom(idx, v)))
      newPos <- dfs(idx, u, pos.updated(idx, v.toByte), toRoom, cost + costs(idx), costs, graph)
    yield newPos
    if !cantStop(u) && (toRoom == isRoom(u)) then
      (cost, pos) +: steps
    else
      steps