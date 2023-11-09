#!/usr/bin/env sh

clear
echo "Who is best pony?"

best_pony="Pinkie Pie"
best_pony_length=$(echo "$best_pony" | awk '{print length}')
cursor=0

while true; do
	stty -icanon -echo
	char=$(dd bs=1 count=1 2> /dev/null)
	stty sane
	case $char in
		"$(printf "\n")")
			if [ ${cursor} -eq "${best_pony_length}" ]; then
				printf "\nThank you for confirming that %s is best pony!\n" "$best_pony"
				break
			fi
			;;
		"$(printf "\177")")
			if [ ${cursor} -gt 0 ]; then
				cursor=$((cursor - 1))
			fi
			;;
		*)
			if [ ${cursor} -lt "${best_pony_length}" ]; then
				cursor=$((cursor + 1))
			fi
			;;
	esac
	display_best_pony=$(echo "$best_pony" | sed "s/^\(.\{$cursor\}\).*/\1/")
	printf "\r\033[K%s" "$display_best_pony"
done
