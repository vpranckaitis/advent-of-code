import java.lang.Integer.parseInt
import java.lang.Math.{abs, max, min}
import scala.annotation.tailrec
import scala.io.{Source, StdIn}

object day08 extends App:
  val input = Source.fromFile("files/08.in")

  val data = input.getLines().map { l =>
    val parts = l.split(""" \| """).map { _.split(" ").toSeq }
    (parts(0), parts(1))
  }.toSeq

  println(part1(data))

  println(part2(data))

  input.close()


  val Wirings: Array[String] = Array(
    "abcefg",
    "cf",
    "acdeg",
    "acdfg",
    "bcdf",
    "abdfg",
    "abdefg",
    "acf",
    "abcdefg",
    "abcdfg",
  )


  def part1(data: Seq[(Seq[String], Seq[String])]): Int =
    data.flatMap { _._2 }.count { s =>
      val l = s.length
      l == 2 || l == 4 || l == 3 || l == 7
    }

  def part2(data: Seq[(Seq[String], Seq[String])]): Int =
    def containsAll(a: String, b: String): Boolean =
      b.forall { a.contains(_) }


    val parsed = data.map { case (signal, output) =>
      val one = signal.find { _.length == 2 }.get
      val four = signal.find { _.length == 4 }.get
      val seven = signal.find { _.length == 3 }.get
      val eight = signal.find { _.length == 7 }.get

      val six = signal.filter { _.length == 6 }.find { !containsAll(_, seven) }.head
      val three = signal.filter { _.length == 5 }.find { containsAll(_, seven) }.head
      val nine = signal.filter { _.length == 6 }.find { containsAll(_, three) }.head
      val zero = signal.filter { _.length == 6 }.filterNot { _ == six }.filterNot { _ == nine }.head
      val five = signal.filter { _.length == 5 }.filterNot { _ == three }.find { containsAll(nine, _) }.head
      val two = signal.filter { _.length == 5 }.filterNot { _ == three }.filterNot { _ == five }.head

      val numbers = Seq(zero, one, two, three, four, five, six, seven, eight, nine).map { _.sorted }.zipWithIndex

      output.map { v => numbers.find { _._1 == v.sorted }.get._2 }.foldLeft(0) { (acc, v) => acc * 10 + v}
    }

    parsed.sum