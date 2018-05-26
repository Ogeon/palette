cd palette
current_version="$(cargo read-manifest | sed 's/.*"version":"\([^"]\+\)".*/\1/g')"
current_date=
cd ..

echo -e "# Changelog\n" > CHANGELOG.md
echo -e "## Version $current_version - $(date +%F)\n" >> CHANGELOG.md

pulls=()
issues=()

git log --pretty="%an<%ae>;%H;%ad;%s" --date=short |
{
	while read line; do
		hash="$(echo "$line" | sed 's/.*;\([^;]*\);.*;.*/\1/g')"
		tags="$(git tag -l --column --points-at "$hash")"
		if [[ $tags =~ [0-9]+\.[0-9]+\.[0-9]+ ]]; then
			date="$(echo "$line" | sed 's/.*;.*;\(.\+\);.*/\1/g')"
			version="$(echo "$tags" | sed 's/\(.*\s\)?\([0-9]+\.[0-9]+\.[0-9]+\)\(\s.*\)?/\1/g')"
			echo -e "\n## Version ${version} - ${date}\n" >> CHANGELOG.md
		elif [[ $line =~ Homu\<homu@barosl.com\>\;.* ]] || [[ $line =~ ^bors\[bot\].* ]]; then
			parts="$(echo "$line" | sed 's/.*;\([^;]*\);.*;.*#\([0-9]*\)*/\1 \2/g')"
			parts=($parts)
			description="$(git log -1 --pretty=format:%b ${parts[0]})"

			if [[ $line =~ ^bors\[bot\].* ]]; then
				description="$(echo "$description" | sed 's/[0-9]*: \(\[.*\]\s*\)\?\(.*\) r=.* a=.*/\2/g')"
			fi

			header="$(echo "$description" | head -n 1)"

			fixes="$(echo "$description" | grep -iEo "(close|closes|closed|fix|fixes|fixed|resolve|resolves|resolved) #[0-9]+" | sed 's/.* #\([0-9]*\)/\1/g')"

			issues+=("$fixes")
			pulls+=("${parts[1]}")

			fixes="$(echo "$fixes" | sed ':a;N;$!ba;s/\n/, /g' | sed 's/\([0-9]\+\)/[#\1][\1]/g')"

			entry="* [#${parts[1]}][${parts[1]}]: $header."

			if [[ "$fixes" != "" ]]; then
				echo "$entry Closes $fixes." >> CHANGELOG.md
			else
				echo "$entry"  >> CHANGELOG.md
			fi
		fi
	done

	echo -e "The first published version.\n" >> CHANGELOG.md

	for id in ${pulls[@]}; do
		echo "[$id]: https://github.com/Ogeon/palette/pull/$id" >> CHANGELOG.md
	done

	for id in ${issues[@]}; do
		echo "[$id]: https://github.com/Ogeon/palette/issues/$id" >> CHANGELOG.md
	done
}
