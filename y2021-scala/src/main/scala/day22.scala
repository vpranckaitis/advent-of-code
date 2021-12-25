import java.lang.Integer.parseInt
import scala.annotation.tailrec
import scala.io.{Source, StdIn}
import scala.util.{Success, Try}

object day22 extends App:
  val input = Source.fromFile("files/22.in")

  val data = input.getLines().map {
    case s"${OnOff(isOn)} x=${ParseInt(x1)}..${ParseInt(x2)},y=${ParseInt(y1)}..${ParseInt(y2)},z=${ParseInt(z1)}..${ParseInt(z2)}" =>
      Command(isOn, x1, x2, y1, y2, z1, z2)
  }.toSeq

  println(part1(data))
  println(part2(data))

  input.close()


  def part1(commands: Seq[Command]): Int =
    val filteredCommands = commands.filter { c =>
      c.x1 >= -50 && c.x2 <= 50 && c.y1 >= -50 && c.y2 <= 50 && c.z1 >= -50 && c.z2 <= 50
    }

    val states = filteredCommands.foldLeft(Map.empty[(Int, Int, Int), Boolean]) { (acc, c) =>
      val updates = for
        x <- c.x1 to c.x2
        y <- c.y1 to c.y2
        z <- c.z1 to c.z2
      yield ((x, y, z), c.on)

      acc ++ updates
    }

    states.values.count(identity)


  def part2(commands: Seq[Command]): Long =
    val xs = commands.flatMap { c => Seq(c.x1, c.x2 + 1) }.distinct.sorted

    val xSpans = xs.sliding(2).map { case Seq(l, r) =>
      val ys = commands.
        withFilter { c => c.x1 <= l && r <= c.x2 + 1 }.
        flatMap { c => Seq(c.y1, c.y2 + 1) }.
        distinct.
        sorted
      val ySpans = ys.sliding(2).map { case Seq(l, r) =>
        val zs = commands.
          withFilter { c => c.y1 <= l && r <= c.y2 + 1 }.
          flatMap { c => Seq(c.z1, c.z2 + 1) }.
          distinct.
          sorted
        val zSpans = zs.sliding(2).map { case Seq(l, r) =>
          Span(l, r, false)
        }.toSeq
        Span(l, r, zSpans)
      }.toSeq
      Span(l, r, ySpans)
    }.toSeq

    val spansAfterCommands = commands.foldLeft(xSpans) { (spans, c) =>
      update(spans, c.x1, c.x2 + 1) { spans =>
        update(spans, c.y1, c.y2 + 1) { spans =>
          update(spans, c.z1, c.z2 + 1) { value =>
            c.on
          }
        }
      }
    }

    sum(spansAfterCommands) { s =>
      (s.end - s.begin) * sum(s.value) { s =>
        (s.end - s.begin) * sum(s.value) { s =>
          if s.value then
            s.end - s.begin
          else
            0
        }
      }
    }


  case class Span[A](begin: Int, end: Int, value: A)

  def update[A](spans: Seq[Span[A]], begin: Int, end: Int)(fn: A => A): Seq[Span[A]] =
    spans.map { s =>
      if s.end <= begin || end <= s.begin then
        s
      else
        Span(s.begin, s.end, fn(s.value))
    }


  def sum[A](spans: Seq[Span[A]])(fn: Span[A] => Long): Long =
    spans.map { fn(_) }.sum


  case class Command(on: Boolean, x1: Int, x2: Int, y1: Int, y2: Int, z1: Int, z2: Int)


  object ParseInt:
    def unapply(s: String): Option[Int] = Try { Integer.parseInt(s) }.toOption


  object OnOff:
    def unapply(s: String): Option[Boolean] =
      if s == "on" then
        Some(true)
      else if s == "off" then
        Some(false)
      else
        None