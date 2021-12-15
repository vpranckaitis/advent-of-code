import java.lang.Integer.parseInt
import java.lang.Math.abs
import scala.annotation.tailrec
import scala.collection.immutable.Queue
import scala.io.{Source, StdIn}
import scala.util.{Success, Try}

object day14 extends App:
  val input = Source.fromFile("files/14.in")

  val lines = input.getLines()
  val formula = lines.next()
  val rules = lines.flatMap {
    case s"$from -> $to" => Some((from, to))
    case _ => None
  }.toMap

  println(part1(formula, rules))
  println(part2(formula, rules))

  input.close()


  @tailrec
  def applyRules(k: Int, formula: String, rules: Map[String, String]): String =
    if k == 0 then
      formula
    else
      val updatedFormula = (formula + '$').sliding(2).map { s =>
        rules.get(s) match
          case Some(r) => s(0) + r
          case None => s
      }.mkString.stripSuffix("$")
      applyRules(k - 1, updatedFormula, rules)


  def part1(formula: String, rules: Map[String, String]): Int =
    val result = applyRules(10, formula, rules)
    val counts = result.groupBy(identity).values.map { _.length }.toSeq
    counts.max - counts.min


  def merge(m1: Map[Char, Long], m2: Map[Char, Long]): Map[Char, Long] =
    (m1.keys ++ m2.keys).map { k => (k, m1.getOrElse(k, 0L) + m2.getOrElse(k, 0L)) }.toMap


  def get(k: Int,
          d: String,
          rules: Map[String, String],
          memo: Map[(String, Int), Map[Char, Long]]
         ): (Map[(String, Int), Map[Char, Long]], Map[Char, Long]) =
    val key = (d, k)
    val updatedMemo = if memo.contains(key) then
      memo
    else if k == 0 then
      memo.updated(key, Map(d(0) -> 1L))
    else
      rules.get(d) match
        case None =>
          memo.updated(key, Map(d(0) -> 1L))
        case Some(r) =>
          val (memo1, counts1) = get(k - 1, d(0) + r, rules, memo)
          val (memo2, counts2) = get(k - 1, r + d(1), rules, memo1)
          memo2.updated(key, merge(counts1, counts2))
    (updatedMemo, updatedMemo(key))


  def part2(formula: String, rules: Map[String, String]): Long =
    val counts = (formula + '$').sliding(2).foldLeft(Map.empty[Char, Long]) { (acc, s) =>
      merge(acc, get(40, s, rules, Map.empty)._2)
    }.values.toSeq

    counts.max - counts.min

