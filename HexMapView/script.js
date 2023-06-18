// Attach our hexmap to <div id="hexmap1"></div>
hex = new OI.hexmap(document.getElementById('hexmap2'),{
	'label':{
		'show': true,	// Show a label
		'clip': true,	// Make sure the label is clipped to the hexagon
		// Define a function to format the hex labels
		// It is passed:
		//  * txt - a text string with the hex's name
		//  * attr - an object containing:
		//		* size - the size in pixels
		//		* font-size - the font size in pixels
		//		* x - the horizontal position in pixels
		//		* y - the vertical position in pixels
		//		* hex - the hexagon's HexJSON properties
		'format': function(txt,attr){
			tspans = '<tspan class="off">'+txt+'</tspan>';
			lines = txt.split(/,/);
			// lines.push(attr.hex.pop.toLocaleString());
			for(var i = 0; i < lines.length; i++){
			    tspans += '<tspan class="on'+(i==lines.length-1 ? ' big':'')+'" y="'+(
                                attr.y + (i-lines.length/2+0.5)*attr['font-size'])+'" x="'+attr.x+'">'+lines[i]+'</tspan>';
			}
                        console.log(tspans);
			return tspans;
		}
	},
	'hexjson':{
		"layout":"odd-r",
		"hexes": {
			"A":{"n":"Something","name":"Something else","q":0,"r":1},
			"B":{"n":"1,1","q":1,"r":1},
			"C":{"n":"2,1","q":2,"r":1},
			"D":{"n":"0,2","q":0,"r":2},
			"E":{"n":"1,2","q":1,"r":2},
			"F":{"n":"2,2","q":2,"r":2}
		}
	}
});
