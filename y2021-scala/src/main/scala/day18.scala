import java.lang.Integer.parseInt
import java.lang.Math.{abs, max}
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.collection.mutable
import scala.io.{Source, StdIn}
import scala.util.{Success, Try}

object day18 extends App:
  val input = Source.fromFile("files/18.in")

  val nodes = input.getLines().map { s => parse(s)._1 }.toSeq

  println(part1(nodes))
  println(part2(nodes))

  input.close()


  def part1(nodes: Seq[Node]): Long =
    val summed = nodes.tail.foldLeft(nodes.head) { (acc, node) =>
      reduce(Pair(acc, node))
    }

    def calculateMagnitude(node: Node): Long =
      node match
        case Value(x) => x
        case Pair(l, r) => calculateMagnitude(l) * 3 + calculateMagnitude(r) * 2

    calculateMagnitude(summed)


  def part2(nodes: Seq[Node]): Long =
    val magnitudes = for
      (l, i) <- nodes.zipWithIndex
      (r, j) <- nodes.zipWithIndex
      if i != j
    yield part1(Seq(l, r))

    magnitudes.max


  @tailrec
  def reduce(node: Node): Node =
    val (explodedNode, hasExploded, _, _) = explode(node, 0)
    if hasExploded then
      reduce(explodedNode)
    else
      val (splitNode, hasSplit) = split(node)
      if hasSplit then
        reduce(splitNode)
      else
        node


  def explode(node: Node, depth: Int): (Node, Boolean, Option[Int], Option[Int]) =
    node match
      case Pair(Value(l), Value(r)) if depth > 3 =>
        (Value(0), true, Some(l), Some(r))
      case node@Value(_) =>
        (node, false, None, None)
      case node@Pair(l, r) =>
        explode(l, depth + 1) match
          case (nodeLeft, true, addLeft, addRight) =>
            val (nodeRight, mergedRight) = mergeRight(r, addRight)
            (Pair(nodeLeft, nodeRight), true, addLeft, mergedRight)
          case (_, false, _, _) =>
            explode(r, depth + 1) match
              case (nodeRight, true, addLeft, addRight) =>
                val (nodeLeft1, mergedLeft) = mergeLeft(l, addLeft)
                (Pair(nodeLeft1, nodeRight), true, mergedLeft, addRight)
              case (_, false, _, _) =>
                (node, false, None, None)


  def mergeLeft(node: Node, value: Option[Int]): (Node, Option[Int]) =
    (node, value) match
      case (node, None) =>
        (node, None)
      case (Value(x), Some(y)) =>
        (Value(x + y), None)
      case (Pair(l, r), value) =>
        val (nodeRight, value1) = mergeLeft(r, value)
        val (nodeLeft, value2) = mergeLeft(l, value1)
        (Pair(nodeLeft, nodeRight), value2)


  def mergeRight(node: Node, value: Option[Int]): (Node, Option[Int]) =
    (node, value) match
      case (node, None) =>
        (node, None)
      case (Value(x), Some(y)) =>
        (Value(x + y), None)
      case (Pair(l, r), value) =>
        val (nodeLeft, value1) = mergeRight(l, value)
        val (nodeRight, value2) = mergeRight(r, value1)
        (Pair(nodeLeft, nodeRight), value2)


  def split(node: Node): (Node, Boolean) =
    node match
      case Pair(l, r) =>
        val (nodeLeft, splitLeft) = split(l)
        val (nodeRight, splitRight) = if splitLeft then
          (r, false)
        else
          split(r)
        if splitLeft || splitRight then
          (Pair(nodeLeft, nodeRight), true)
        else
          (node, false)
      case Value(x) if x > 9 =>
        val l = x / 2
        val r = x - l
        (Pair(Value(l), Value(r)), true)
      case node =>
        (node, false)


  trait Node
  case class Value(x: Int) extends Node
  case class Pair(l: Node, r: Node) extends Node


  def parse(s: String): (Node, String) =
    if s(0) == '[' then
      val (l, rest1) = parse(s.substring(1))
      val (r, rest2) = parse(rest1.substring(1))
      (Pair(l, r), rest2.substring(1))
    else
      def indexOf(s: String, c: Char) =
        val idx = s.indexOf(c)
        if idx == -1 then s.length else idx

      val idx = Math.min(indexOf(s, ','), indexOf(s, ']'))
      val x = Integer.parseInt(s.substring(0, idx))
      (Value(x), s.substring(idx))
