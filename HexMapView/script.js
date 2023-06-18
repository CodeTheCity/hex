// Attach our hexmap to <div id="hexmap1"></div>
console.log(JSON.stringify(mapData));
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
	'hexjson': mapData
});

// Define a colour scale helper function
function ColourScale(c){
	var s,n;
	s = c;
	n = s.length;
	// Get a colour given a value, and the range minimum/maximum
	this.getValue = function(v,min,max){
		var c,a,b;
		v = (v-min)/(max-min);
		if(v<0) return 'rgb('+s[0].rgb.join(',')+')';
		if(v>=1) return 'rgb('+s[n-1].rgb.join(',')+')';
		for(c = 0; c < n-1; c++){
			a = s[c];
			b = s[c+1];
			if(v >= a.v && v < b.v){
				pc = Math.min(1,(v - a.v)/(b.v-a.v));
				rgb = [Math.round(a.rgb[0] + (b.rgb[0]-a.rgb[0])*pc),Math.round(a.rgb[1] + (b.rgb[1]-a.rgb[1])*pc),Math.round(a.rgb[2] + (b.rgb[2]-a.rgb[2])*pc)];
				return 'rgb('+rgb.join(',')+')';
			}
		}
	};
	return this;
}

// Define the Viridis colour scale
viridis = new ColourScale([{'rgb':[68,1,84],v:0},{'rgb':[72,35,116],'v':0.1},{'rgb':[64,67,135],'v':0.2},{'rgb':[52,94,141],'v':0.3},{'rgb':[41,120,142],'v':0.4},{'rgb':[32,143,140],'v':0.5},{'rgb':[34,167,132],'v':0.6},{'rgb':[66,190,113],'v':0.7},{'rgb':[121,209,81],'v':0.8},{'rgb':[186,222,39],'v':0.9},{'rgb':[253,231,36],'v':1}]);
// const mapDataInfo = require('../Data/aberdeenshire_IZ.json');
// Create the hexagon layout
hex = new OI.hexmap(document.getElementById('hexmap3'),{
	// The HexJSON layout
	'hexjson':mapData,
	// Once we've loaded the map the ready function is called
	'ready':function(){
		// Load the data
		OI.ajax('../Data/male_aberdeenshire_IZ.json',{
			'this': this, // Set the context to the hexmap
			'dataType':'json',
			'success':function(data){
				// Process the data file to find the minimum and maximum
				var min = 1e100;
				var max = -1e100;
				for(var r in data){
					min = Math.min(data[r],min);
					max = Math.max(data[r],max);
				}
				this.data = data;
				// Update the hex colours
				this.updateColours(function(r){ return viridis.getValue(data[r],min,max); });
			},
			'error':function(e,attr){ this.log('ERROR','Unable to load ',attr.url,attr); }
		});
	}
});

// Make a tooltip
hex.on('mouseover',function(e){
	var svg,tip,bb,bbo,hex;
	svg = e.data.hexmap.el;
	hex = e.target;
	// Get any existing tooltip for this hexmap
	tip = svg.querySelector('.tooltip');
	console.log(e.data.region);
	if(!tip){
		// Add a new tooltip
		tip = document.createElement('div');
		tip.classList.add('tooltip');
		svg.appendChild(tip);
	}
	// Update contents of tooltip
	tip.innerHTML = e.data.data.n+'<br />'+mapData.hexes[e.data.region].clients_per_100_pop.toLocaleString()+' clients per 100 population';
	// Update position of tooltip
	bb = hex.getBoundingClientRect();
	bbo = svg.getBoundingClientRect();
	tip.style.left = Math.round(bb.left + bb.width/2 - bbo.left + svg.scrollLeft)+'px';
	tip.style.top = Math.round(bb.top + bb.height/2 - bbo.top)+'px';
});