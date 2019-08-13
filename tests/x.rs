#![feature(proc_macro_hygiene)]

use format_xml::format_xml;

#[test]
fn x() {
	let x = 10;
	let y = 20;

	let s = format_xml! {

<svg height={720} width={1280}>
	<line x1="0" y1=0 x2={x} y2={y} />
	"inline &<>"<br>
	{42}
</svg>

	}.to_string();

	assert_eq!(s, "<svg height=\"720\" width=\"1280\"><line x1=\"0\" y1=\"0\" x2=\"10\" y2=\"20\" />inline &amp;&lt;&gt;<br>42</svg>");
}

#[test]
fn readme() {
	let point = (20, 30);
	let name = "World";

	let s = format_xml! {
		<svg width=200 height=200>
			<line x1=0 y1=0 x2={point.0} y2={point.1} stroke="black" stroke-width=2 />
			<text x={point.1} y={point.0}>"Hello '" {name} "'!"</text>
		</svg>
	}.to_string();

	assert_eq!(s, "<svg width=\"200\" height=\"200\"><line x1=\"0\" y1=\"0\" x2=\"20\" y2=\"30\" stroke=\"black\" stroke-width=\"2\" /><text x=\"30\" y=\"20\">Hello &apos;World&apos;!</text></svg>");
}
