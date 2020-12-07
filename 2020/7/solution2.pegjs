Main = rules1:(Rule "\n"?)*  "\n"* {
	const rules =  rules1.reduce((acc, el) => {
		acc[el[0].color] = el[0].contents;
		return acc;
	}, {});

	// dfs
	const search = (graph, cur, visited) => {
		let sum = 0;
		for (let idx in graph[cur]) {
			const x = graph[cur][idx];

			if (!visited[x.color]) {
				visited[x.color] = search(graph, x.color, visited);
			}
			sum += x.count * visited[x.color];
		}
		return sum + 1;
	};

	return search(rules, "shiny gold", {}) - 1;
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
