
# Riichi Mahjong Calculator Via the Terminal

# ![demo gif](demo.gif)

Options
===============

```
--help
--tiles   tiles                (default: none)
-w --win     winning tile         (default: last tile in hand)
-d --dora    dora (and ura) tiles (default: none)
-s --seat    seat wind            (default: East)
-p --prev    prevelant wind       (default: East)
-t --tsumo   tsumo                (default: false)
-r --riichi  riichi               (default: false)
-m --manual  calc mode            (default: false)
-b --ba      honba count          (default: 0)
.
.
.
.
and so on and so on 
```

Notation
-------
- man:  1m, 2m, 3m, 4m, 5m, 6m, 7m, 8m, 9m
- pin:  1p, 2p, 3p, 4p, 5p, 6p, 7p, 8p, 9p
- sou:  1s, 2s, 3s, 4s, 5s, 6s, 7s, 8s, 9s
- wind: Ew, Sw Ww, Nw (east, south, west, north)
- drgn: rd, gd, wd (red, green, white) 
- o: signifies an open set (eg: 234po)


#### Examples

##### Calculator Mode
```
~/$ mahc -m 4 30 --ba 3
> Dealer:    12500 (4200) 
  non-dealer: 8600 (2300/ 4200)
```



##### Normal Mode NOT FULLY YET IMPLEMENTED (some yaku missing)
note: the winning group has to go last (this is to calculate fu correctly)
``` 
~/$ mahc --tiles rrrd EEEw 234p 234p 11p -w 1p -p Ew -s Ew

7 Han/ 50 Fu
Dealer: 18000 (6000)
Non-dealer: 12000 (3000/6000)
Yaku:
  Iipeikou: 1
  Honitsu: 3
  Yakuhai: 1
  Yakuhai: 1
  Yakuhai: 1

Fu:
  BasePoints: 20
  ClosedRon: 10
  NonSimpleClosedTriplet: 8
  NonSimpleClosedTriplet: 8
  SingleWait: 2

```

##### implemented hand validations as of yet

###### One Han Yaku
- [x] Tanyao
- [x] Iipeikou 
- [x] Yakuhai 
- [ ] MenzenTsumo
- [ ] Pinfu
- [ ] Riichi
- [ ] Ippatsu
- [ ] HaiteiRaoyue
- [ ] HouteiRaoyui
- [ ] RinshanKaihou
- [ ] Chankan

###### Two Han Yaku
- [ ] DoubleRiichi
- [x] Toitoi
- [ ] Ittsuu
- [x] SanshokuDoujun
- [ ] Chantaiyao
- [x] Sanankou
- [ ] SanshokuDoukou
- [ ] Sankantsu
- [ ] Honroutou
- [ ] Shousangen
- [ ] Chiitoitsu

###### Three Han Yaku
- [x] Honitsu
- [ ] JunchanTaiyao
- [x] Ryanpeikou 

###### Six Han Yaku
- [ ] Chinitsu

###### Yakuman 
- [ ] KazoeYakuman
- [ ] KokushiMusou
- [ ] Suuankou
- [ ] Daisangen
- [ ] Shousuushii
- [ ] Daisuushii
- [ ] Tsuuiisou
- [ ] Chinroutou
- [ ] Ryuuiisou
- [ ] ChuurenPoutou
- [ ] Suukantsu
- [ ] Tenhou
- [ ] Chiihou


# ![this.jpg](https://64.media.tumblr.com/07006d83e5810b3c651254e7b9a3e713/c4dc091a7806e504-ef/s400x600/cdfb08014450e71074a0a8763a67661485d59f8c.gif)
