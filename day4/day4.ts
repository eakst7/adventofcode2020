import * as fs from 'fs'

function readInputData() : string[] {
  const data: string = fs.readFileSync('input.txt', 'utf8')
  var lines: string[] = data.split('\n')

  return lines
}

class Passport {
  public fields: Object = {}

  constructor(line: string) {
    let pairs = line.split(" ")
    for (let pair of pairs) {
      let keyvalue = pair.split(":")
      this.fields[keyvalue[0]] = keyvalue[1]
    }
  }

  public hasAllRequiredFields() : boolean {
    return (
      ('byr' in this.fields) &&
      ('iyr' in this.fields) &&
      ('eyr' in this.fields) &&
      ('hgt' in this.fields) &&
      ('hcl' in this.fields) &&
      ('ecl' in this.fields) &&
      ('pid' in this.fields)
      )
  }

  private checkHgt(hgt: string) : boolean {
    let hgt_m = hgt.match(/(\d+)(.*)/)
    let hgt_v = Number(hgt_m[1])
    let hgt_u = hgt_m[2]

    if ((hgt_u === 'cm') && (hgt_v >= 150) && (hgt_v <= 193)) {
      return true
    }
    if ((hgt_u === 'in') && (hgt_v >= 59) && (hgt_v <= 76)) {
      return true
    }

    return false
  }

  public allFieldsAreValid() : boolean {

    try {
      let byr = Number(this.fields['byr'])
      if ((byr < 1920) || (byr > 2002)) return false

      let iyr = Number(this.fields['iyr'])
      if ((iyr < 2010) || (iyr > 2020)) return false

      let eyr = Number(this.fields['eyr'])
      if ((eyr < 2020) || (eyr > 2030)) return false

      if (!this.checkHgt(this.fields['hgt'])) return false

      let match = this.fields['hcl'].match(/^#[0-9a-f]{6}$/)
      if (!match) return false

      match = this.fields['ecl'].match(/^(amb|blu|brn|gry|grn|hzl|oth)$/)
      if (!match) return false

      match = this.fields['pid'].match(/^\d{9}$/)
      if (!match) return false

      return true
    } catch {
      return false
    }

  }

}

function joinLines(lines: string[]) {
  let entries : string[] = []
  let currentEntry : string = ""
  for (let l of lines) {
    if (l === "") {
      entries.push(currentEntry.trim())
      currentEntry = ""
    } else {
      currentEntry += l + " "
    }
  }

  return entries
}

function findSolutionPart1(lines: string[]) {
  let passportEntries = joinLines(lines)

  let validCount = 0
  for (let entry of passportEntries) {
    let passport = new Passport(entry)
    if (passport.hasAllRequiredFields()) {
      validCount++
    }
  }

  console.log(validCount)
}

function findSolutionPart2(lines: string[]) {
  let passportEntries = joinLines(lines)

  let validCount = 0
  for (let entry of passportEntries) {
    let passport = new Passport(entry)
    if (passport.hasAllRequiredFields() && passport.allFieldsAreValid()) {
      validCount++
    }
  }

  console.log(validCount)
}

let lines: string[] = readInputData()
findSolutionPart1(lines)
findSolutionPart2(lines)
