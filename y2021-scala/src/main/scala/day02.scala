import java.lang.Integer.parseInt
import scala.io.{Source, StdIn}

object day02 extends App:
  val input = Source.fromFile("files/02.in")

  val commands = input.getLines().toList

  val deltas1 = commands.map {
    case s"forward $x" => (parseInt(x), 0)
    case s"down $x" => (0, parseInt(x))
    case s"up $x" => (0, -parseInt(x))
  }

  val pos1 = deltas1.foldLeft((0, 0)) { case ((x, y), (dx, dy)) =>
    (x + dx, y + dy)
  }

  println(pos1._1 * pos1._2)

  val pos2 = commands.foldLeft((0, 0, 0)) { case ((x, y, a), c) =>
    c match {
      case s"forward $v" =>
        val dx = parseInt(v)
        (x + dx, y + dx * a, a)
      case s"down $v" =>
        val da = parseInt(v)
        (x, y, a + da)
      case s"up $v" =>
        val da = parseInt(v)
        (x, y, a - da)
    }
  }

  println(pos2._1 * pos2._2)

  input.close()