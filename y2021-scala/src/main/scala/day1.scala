import scala.io.{Source, StdIn}

object day1 extends App:
  val input = Source.fromFile("files/01.in")

  val depths = input.getLines().map { s =>
    Integer.parseInt(s)
  }.toList

  val res1 = depths.sliding(2, 1).count {
    case Seq(x, y) => x < y
  }

  println(res1)

  val res2 = depths.sliding(3, 1).map { _.sum }.sliding(2, 1).count {
    case Seq(x, y) => x < y
  }

  println(res2)

  input.close()