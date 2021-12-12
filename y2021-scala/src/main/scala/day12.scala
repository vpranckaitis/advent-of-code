import java.lang.Integer.parseInt
import java.lang.Math.abs
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.io.{Source, StdIn}

object day12 extends App:
  val Start = "start"
  val End = "end"

  val input = Source.fromFile("files/12.in")

  val data: Seq[(String, String)] = input.getLines().flatMap { l =>
    l.split("-").toList match
      case u :: v :: Nil => Seq((u, v), (v, u))
  }.toSeq

  val graph = data.groupBy { _._1 }.view.mapValues { _.map { _._2 } }.toMap

  println(part1(graph))
  println(part2(graph))

  input.close()


  def part1(g: Map[String, Seq[String]]): Int =
    def traverse(u: String, visited: Set[String]): Int =
      if u == End then
        1
      else
        g(u).filterNot(visited).map { v =>
          val updatedVisited = if v.forall { _.isLower } then
            visited + v
          else
            visited

          traverse(v, updatedVisited)
        }.sum

    traverse(Start, Set(Start))


  def part2(g: Map[String, Seq[String]]): Int =
    def traverse(u: String, visited: Set[String], canRepeat: Boolean): Int =
      if u == End then
        1
      else
        g(u).flatMap { v =>
          val (canEnter, updatedVisited, updatedCanRepeat) =
            if !v.forall { _.isLower } then
              (true, visited, canRepeat)
            else if !visited.contains(v) then
              (true, visited + v, canRepeat)
            else if canRepeat && v != Start then
              (true, visited, false)
            else
              (false, visited, false)

          if canEnter then
            Some(traverse(v, updatedVisited, updatedCanRepeat))
          else
            None
        }.sum

    traverse(Start, Set(Start), true)




