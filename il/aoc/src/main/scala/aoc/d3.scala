package aoc
import scala.util.matching.Regex
import scala.collection.mutable.ListBuffer

object D3 {
  lazy val i: String = scala.io.Source
    .fromFile(
      s"${System.getProperty("user.dir")}/src/main/resources/aoc/d3.txt"
    )
    .mkString
    .trim

  object P1 {
    private val regex: Regex = """mul\((-?\d+),(-?\d+)\)""".r
    def getMultiples(input: String) = regex
      .findAllMatchIn(input)
      .map(m => m.group(1).toInt * m.group(2).toInt)
      .sum
  }

  def main(args: Array[String]): Unit = {
    println(P1.getMultiples(i))
    println(P2.getMultiples(i))
  }
  object P2 {
    private val regex = """mul\((-?\d+),(-?\d+)\)|do(n't)?\(\)""".r

    def getMultiples(input: String): Int = {
      var wasLastDo = true
      return regex
        .findAllMatchIn(input)
        .map { m =>
          (m.matched) match {
            case "do()" => {
              wasLastDo = true
              0
            }
            case "don't()" => {
              wasLastDo = false
              0
            }
            case _ => {

              if (wasLastDo) (m.group(1).toInt * m.group(2).toInt) else 0
            }
          }
        }
        .sum
    }
  }

}
