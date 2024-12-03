from sys import stdin
#12 red cubes, 13 green cubes, and 14 blue
def main():
    ans = 0
    game_num = 0
    for game in stdin:
        game_num += 1
        is_game_possible = True
        game  = game.replace("\n","")
        res = game.split(" ")
        res = res[2::1]
        res = ",".join(res)
        res = res.split(";,")
        #print(res, end="\n\n")
        for inst in res:
            inst = inst.split(",")
            inst = [x for x in inst if x != ""]
            print(inst)
            for i in range(0, len(inst), 2):
                if is_game_possible == False:
                    continue
                number = int(inst[i])
                color = inst[i+1] # oh god please dont have index outbound error
                if color == 'red':
                    is_game_possible = number < 12
                if color == 'green':
                    is_game_possible = number < 13
                if color == 'blue':
                    is_game_possible = number < 14
        if is_game_possible:
            ans += game_num
            print(f"game_num = {game_num}")

        print()
        print()
    print(f"ans = {ans}")
        

main()
