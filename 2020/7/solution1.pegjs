Main = rules1:(Rule "\n"?)*  "\n"* {
	const rules =  rules1.reduce((acc, el) => {
		acc[el[0].color] = el[0].contents;
		return acc;
	}, {});

	const reverse = (rules, reversed) => {
		for (let color in rules) {
			rules[color].map((el) => {
				if (!reversed[el.color]) {
					reversed[el.color] = [];
				}

				reversed[el.color].push({
					color,
					count: el.count,
				});
			});
		}
	}

	const reversed = {};
	reverse(rules, reversed);

	// dfs
	const search = (graph, cur, visited) => {
		for (let idx in graph[cur]) {
			const x = graph[cur][idx];

			if (!visited.has(x.color)) {
				search(graph, x.color, visited)
				visited.add(x.color);
			}
		}
	};

	const visited = new Set();
	search(reversed, "shiny gold", visited);

	return [...visited.keys()].length;
}

Rule = color:Color _ "bags contain" _ contents:Contents {
	const rule = {color, contents};
	return rule
}

Contents
	= head:(Content "," _)* tail:Content "." {
		return head.map(el => el[0]).concat([tail]);
	}
	/ "no other bags." { return []; }

Content = count:Num _ color:Color _ "bag" "s"? { return {count, color}; }

Num = [0-9]+ { return Number(text()); }

Color = [a-z]+ _ [a-z]+ { return text(); }

_ = " "
