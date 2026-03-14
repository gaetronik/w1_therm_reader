use crate::ResultW1;
use nom::{
    bytes::complete::tag, character::complete::alphanumeric1, character::complete::hex_digit1,
    character::complete::newline, character::complete::not_line_ending, multi::separated_list1,
    sequence::preceded, IResult, Parser,
};

fn parser_hex(input: &str) -> IResult<&str, &str> {
    hex_digit1(input)
}

fn parser_array_hex(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(" "), parser_hex).parse(input)
}

fn parser_end_first_line(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(tag(" : crc="), separated_list1(tag(" "), alphanumeric1)).parse(input)
}

fn parser_temp(input: &str) -> IResult<&str, i32> {
    let (input, (_, temp)) = (tag(" t="), not_line_ending).parse(input)?;
    Ok((input, temp.parse().unwrap()))
}

fn parser_second_line<'a>(input: &'a str, raw_value: &str) -> IResult<&'a str, i32> {
    let (input, (_, _, temp)) = (newline, tag(raw_value), parser_temp).parse(input)?;
    Ok((input, temp))
}

pub fn parser(input: &str) -> IResult<&str, ResultW1> {
    let (input, raw_values) = parser_array_hex(input)?;
    let raw_value = raw_values.join(" ");
    let (input, crc_elems) = parser_end_first_line(input)?;
    let crc_value = crc_elems[1];
    let (input, temp) = parser_second_line(input, &raw_value)?;
    Ok((
        input,
        ResultW1 {
            crc: crc_value == "YES",
            temp,
        },
    ))
}
