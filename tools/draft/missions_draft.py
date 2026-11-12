#!/usr/bin/env python3
"""Writes data/missions.json, the first draft of the 50 missions (journal: Missions).
Each row: number, id, name, kind, map, loadout, requirements, goal, medal measure and thresholds, foes.
A requirement is "N" (clear mission N) or "N:silver" / "N:gold". A mission's tier is how many
requirements it has (Vagrancy's road rule); the finale overrides it with "row"."""
import json, sys
from pathlib import Path

M = []
def m(n, mid, name, kind, map_, loadout, req, goal, measure, b, s, g, foes, brief, **extra):
    M.append(dict(n=n, id=mid, name=name, kind=kind, map=map_, loadout=loadout,
                  requires=req, goal=goal, medals=dict(measure=measure, bronze=b, silver=s, gold=g),
                  foes=foes, brief=brief, **extra))

# goal kinds: break_targets, reach_end, survive{s}, defeat{n}, win_rounds{n}, team_score{n}, clear_waves, defeat_boss, salvage{relics, need}, gauntlet
# measures: time (s, lower better), health (% left, higher better), relics (count, higher better), kills (higher better)
# --- tier 0: open from the start (5)
m(1,"first_light","First Light","targets","plaza","pistol",[],{"kind":"break_targets","targets":10,"limit":90},"time",90,45,28,[],"Ten chrome targets round the Plaza. Pistol only.")
m(2,"plaza_run","Plaza Run","parkour","plaza","none",[],{"kind":"reach_end","gates":6,"limit":90},"time",90,40,26,[],"Six gates over the rooftops. Hop to keep your speed.")
m(3,"doors_open","Doors Open","horde","mercado","any",[],{"kind":"survive","seconds":60},"health",1,40,75,[{"zombie":"shambler","waves":"rising"}],"The Mercado's doors are open. Last one minute.")
m(4,"sparring","Sparring","combat","galeria","any",[],{"kind":"win_rounds","rounds":3},"time",240,120,70,[{"bot":"hidalgo","level":1}],"One opponent, first to three.")
m(5,"steel_etiquette","Steel Etiquette","melee","alcazar","katana",[],{"kind":"win_rounds","rounds":3},"time",240,120,70,[{"bot":"torero","level":1,"weapon":"katana"}],"Katanas only. Parry the lunge.")
# --- tier 1: one requirement (9)
m(6,"bell_tower","Bell Tower","targets","catedral","rifle",["1"],{"kind":"break_targets","targets":12,"limit":120},"time",120,60,38,[],"Twelve targets in the nave and the tower. Rifle only.")
m(7,"toboggan","Tobogán","parkour","tobogan","none",["2"],{"kind":"reach_end","gates":8,"limit":120},"time",120,55,38,[],"Surf the ramps. Never touch the floor.")
m(8,"forty_hands","Forty Hands","horde","mercado","any",["3"],{"kind":"defeat","count":40,"limit":240},"time",240,140,95,[{"zombie":"shambler","waves":"rising"}],"Defeat forty zombies.")
m(9,"two_at_once","Two at Once","combat","galeria","any",["4"],{"kind":"win_rounds","rounds":3},"time",300,150,90,[{"bot":"hidalgo","level":1},{"bot":"infanta","level":1}],"Two opponents, both after you.")
m(10,"shotgun_wedding","Shotgun Wedding","fixed","mercado","shotgun",["4"],{"kind":"win_rounds","rounds":3},"time",300,150,90,[{"bot":"harlequin","level":2,"weapon":"shotgun"}],"Shotguns only, close quarters.")
m(11,"sentinel_row","Sentinel Row","arena","alcazar","any",["4"],{"kind":"clear_waves","waves":3},"time",240,130,85,[{"enemy":"sentinel"},{"enemy":"drifter"},{"enemy":"moth"}],"Three waves of the Waiting City's foes. Their orbs can be shot down only when gold.")
m(12,"navaja_night","Navaja Night","melee","arrabal","navaja",["5"],{"kind":"win_rounds","rounds":3},"time",300,150,90,[{"bot":"sordo","level":2,"weapon":"navaja"}],"Knives in the Arrabal.")
m(13,"salvage_puerto","Salvage: Puerto","salvage","puerto","any",["2"],{"kind":"salvage","relics":8,"need":4,"limit":150},"relics",4,6,8,[{"zombie":"shambler","waves":"guard"}],"Eight relics in the harbour. Four open the lift; take the rest if you dare.")
m(14,"two_by_two","Two by Two","team","plaza","any",["4"],{"kind":"team_score","score":10,"limit":300},"time",300,180,120,[{"bot":"infanta","level":1,"side":"ally"},{"bot":"hidalgo","level":1},{"bot":"friar","level":1}],"You and the Infanta against two. First to ten.")
# --- tier 2: two requirements (9)
m(15,"moving_targets","Moving Targets","targets","acueducto","drum",["6","7"],{"kind":"break_targets","targets":14,"limit":120,"moving":True},"time",120,70,48,[],"Targets ride the aqueduct. The Drum's shells fall.")
m(16,"wall_to_wall","Wall to Wall","parkour","barranco","none",["7","2:silver"],{"kind":"reach_end","gates":8,"limit":120},"time",120,60,42,[],"Three wall jumps a flight. The ravine has no floor.")
m(17,"night_market","Night Market","horde","mercado","any",["8","10"],{"kind":"survive","seconds":120},"health",1,35,70,[{"zombie":"shambler","waves":"rising"},{"zombie":"runner","from":40}],"Runners join at forty seconds.")
m(18,"ladder_one","Ladder I","combat","molino","any",["9","14"],{"kind":"win_rounds","rounds":3,"ladder":3},"time",420,240,160,[{"bot":"hidalgo","level":2},{"bot":"infanta","level":2},{"bot":"harlequin","level":2}],"Three duels in a row. Your health carries over.")
m(19,"trabuco_only","Trabuco Only","fixed","puerto","trabuco",["10","9"],{"kind":"win_rounds","rounds":3},"time",300,160,100,[{"bot":"sordo","level":2,"weapon":"trabuco"},{"bot":"friar","level":2,"weapon":"trabuco"}],"Blunderbusses on the quay.")
m(20,"lantern_hall","Lantern Hall","arena","catedral","any",["11","13"],{"kind":"clear_waves","waves":4},"time",300,170,110,[{"enemy":"lantern"},{"enemy":"bloom"},{"enemy":"moth"}],"Lanterns hang in the nave and bloom.")
m(21,"harvest","Harvest","melee","dehesa","guadana",["12","8"],{"kind":"defeat","count":30,"limit":180},"time",180,110,75,[{"zombie":"shambler","waves":"rising"}],"A guadaña and thirty zombies.")
m(22,"salvage_molino","Salvage: Molino","salvage","molino","any",["13","7"],{"kind":"salvage","relics":10,"need":5,"limit":180},"relics",5,8,10,[{"zombie":"runner","waves":"guard"}],"Ten relics in the mill's sails and cellars.")
m(23,"three_by_three","Three by Three","team","galeria","any",["14","9"],{"kind":"team_score","score":15,"limit":360},"time",360,220,150,[{"bot":"infanta","level":2,"side":"ally"},{"bot":"friar","level":2,"side":"ally"},{"bot":"hidalgo","level":2},{"bot":"sordo","level":2},{"bot":"torero","level":2}],"Three a side. First to fifteen.")
# --- tier 3: three requirements (9)
m(24,"pole_vault","Garrocha","targets","sierra","garrocha",["15","16","10"],{"kind":"break_targets","targets":10,"limit":150},"time",150,90,60,[],"The targets are out of reach. The Garrocha flings you to them.")
m(25,"sierra_ascent","Sierra Ascent","parkour","sierra","none",["16","7:gold","22"],{"kind":"reach_end","gates":10,"limit":150},"time",150,80,55,[],"Ten gates up the mountain.")
m(26,"hundred_hands","Hundred Hands","horde","arrabal","any",["17","21","8"],{"kind":"defeat","count":100,"limit":360},"time",360,240,170,[{"zombie":"shambler","waves":"rising"},{"zombie":"runner","from":20},{"zombie":"brute","from":60}],"A hundred zombies. Brutes come at sixty.")
m(27,"ladder_two","Ladder II","combat","acueducto","any",["18","19","14"],{"kind":"win_rounds","rounds":3,"ladder":4},"time",540,330,220,[{"bot":"sordo","level":3},{"bot":"friar","level":3},{"bot":"torero","level":3},{"bot":"black_knight","level":3}],"Four duels in a row, sharper each time.")
m(28,"black_knight","The Black Knight","boss","alcazar","any",["19","21","12"],{"kind":"defeat_boss"},"time",300,170,110,[{"boss":"black_knight"}],"A knight twice your height, in Chumworld plate. Melee hurts it most from behind.")
m(29,"clerks_office","Clerk's Office","arena","galeria","any",["20","11:gold","17"],{"kind":"clear_waves","waves":4},"time",300,180,120,[{"enemy":"clerk"},{"enemy":"sentinel"},{"enemy":"warden"}],"Clerks blink between desks and stamp the floor.")
m(30,"painter","The Painter","fixed","galeria","pincel",["19","15","23"],{"kind":"win_rounds","rounds":3},"time",300,170,110,[{"bot":"harlequin","level":3,"weapon":"pincel"},{"bot":"infanta","level":3,"weapon":"pincel"}],"Pinceles only in the gallery.")
m(31,"salvage_catedral","Salvage: Catedral","salvage","catedral","any",["22","13:silver","20"],{"kind":"salvage","relics":12,"need":6,"limit":200},"relics",6,9,12,[{"enemy":"lantern"},{"enemy":"moth"}],"Twelve relics in the vaults, guarded by lanterns.")
m(32,"hold_the_plaza","Hold the Plaza","team","plaza","any",["23","18","14:gold"],{"kind":"team_score","score":20,"limit":420},"time",420,280,190,[{"bot":"infanta","level":3,"side":"ally"},{"bot":"friar","level":3,"side":"ally"},{"bot":"hidalgo","level":3,"side":"ally"},{"bot":"sordo","level":3},{"bot":"torero","level":3},{"bot":"harlequin","level":3},{"bot":"black_knight","level":3}],"Four a side in the Plaza.")
# --- tier 4: four requirements (8)
m(33,"cube","The Cube","parkour","cubo","none",["25","16","24","7:gold"],{"kind":"reach_end","gates":12,"limit":180},"time",180,100,70,[],"Every face of the Cubo is a floor. Twelve gates across them.")
m(34,"last_bus","Last Bus","horde","puerto","any",["26","21","17","29"],{"kind":"survive","seconds":180},"health",1,30,60,[{"zombie":"shambler","waves":"rising"},{"zombie":"runner","from":30},{"zombie":"brute","from":90}],"Three minutes until the boat.")
m(35,"ladder_three","Ladder III","combat","sierra","any",["27","30","32","28"],{"kind":"win_rounds","rounds":3,"ladder":5},"time",660,420,290,[{"bot":"torero","level":4},{"bot":"black_knight","level":4},{"bot":"ivory_knight","level":4},{"bot":"sordo","level":4},{"bot":"harlequin","level":4}],"Five duels. The knights fight like you do.")
m(36,"wardens_court","Warden's Court","arena","plaza","any",["29","20","26","31"],{"kind":"clear_waves","waves":5},"time",360,230,160,[{"enemy":"warden"},{"enemy":"bloom"},{"enemy":"drifter"},{"enemy":"clerk"}],"Wardens send waves along the ground. Jump them.")
m(37,"section_chief","The Section Chief","boss","molino","any",["27","32","29","23"],{"kind":"defeat_boss"},"time",360,220,150,[{"boss":"section_chief"}],"A gunman who calls for help. Keep him alone.")
m(38,"long_shot","Long Shot","targets","sierra","rifle",["24","30","15","6:gold"],{"kind":"break_targets","targets":8,"limit":120},"time",120,75,50,[],"Eight targets across the valley.")
m(39,"punal_ballet","Puñal Ballet","melee","selva","punal",["28","21","12","24"],{"kind":"win_rounds","rounds":3},"time",300,180,120,[{"bot":"infanta","level":4,"weapon":"punal"},{"bot":"torero","level":4,"weapon":"punal"}],"High-jumping knives among the vines.")
m(40,"gauntlet_one","Gauntlet I","gauntlet","alcazar","any",["28","29","27","26"],{"kind":"gauntlet","stages":5,"repairs":3},"health",1,40,70,[{"stage":"horde"},{"stage":"combat"},{"stage":"arena"},{"stage":"combat"},{"boss":"black_knight"}],"Five fights, one health bar, three repair cells.")
# --- tier 5: five requirements (8)
m(41,"jungle_line","Jungle Line","parkour","selva","none",["33","25","16","39","7:gold"],{"kind":"reach_end","gates":12,"limit":180},"time",180,110,80,[],"Vines, ramps and a long fall. Twelve gates.")
m(42,"endless_night","Endless Night","horde","dehesa","any",["34","26","17","40","21"],{"kind":"defeat","count":250,"limit":600},"time",600,420,300,[{"zombie":"shambler","waves":"rising"},{"zombie":"runner","from":10},{"zombie":"brute","from":40}],"Two hundred and fifty.")
m(43,"ladder_four","Ladder IV","combat","cubo","any",["35","27","32","39","37"],{"kind":"win_rounds","rounds":3,"ladder":6},"time",800,520,360,[{"bot":"hidalgo","level":5},{"bot":"infanta","level":5},{"bot":"harlequin","level":5},{"bot":"sordo","level":5},{"bot":"friar","level":5},{"bot":"torero","level":5}],"Six painted duelists at their best.")
m(44,"bloom_garden","Bloom Garden","arena","dehesa","any",["36","29","31","34","20"],{"kind":"clear_waves","waves":5},"time",400,260,180,[{"enemy":"bloom"},{"enemy":"lantern"},{"enemy":"warden"},{"enemy":"moth"}],"Blooms open in the meadow.")
m(45,"great_horned","The Great Horned","boss","sierra","any",["36","37","28","38","34"],{"kind":"defeat_boss"},"time",420,260,180,[{"boss":"great_horned"}],"An owl the size of a house. Talons, feathers, a screech.")
m(46,"salvage_alcazar","Salvage: Alcázar","salvage","alcazar","any",["31","22","36","33","13:gold"],{"kind":"salvage","relics":15,"need":8,"limit":240},"relics",8,12,15,[{"enemy":"clerk"},{"zombie":"runner","waves":"guard"}],"Fifteen relics in the fortress.")
m(47,"total_war","Total War","team","cubo","any",["32","35","23","37","30"],{"kind":"team_score","score":25,"limit":480},"time",480,330,230,[{"bot":"ivory_knight","level":5,"side":"ally"},{"bot":"infanta","level":5,"side":"ally"},{"bot":"friar","level":5,"side":"ally"},{"bot":"black_knight","level":5},{"bot":"torero","level":5},{"bot":"sordo","level":5},{"bot":"harlequin","level":5}],"Four a side on the Cubo.")
m(48,"gauntlet_two","Gauntlet II","gauntlet","catedral","any",["40","35","36","37","34"],{"kind":"gauntlet","stages":6,"repairs":3},"health",1,35,65,[{"stage":"arena"},{"stage":"horde"},{"stage":"combat"},{"stage":"arena"},{"stage":"combat"},{"boss":"section_chief"}],"Six fights, one health bar.")
# --- tier 6: the finale and the secret
m(49,"interdictor","The Interdictor","boss","catedral","any",["45","44","43","42","41","48"],{"kind":"defeat_boss"},"time",480,320,220,[{"boss":"bailiff"}],"Long Time Comin's first judge. Three phases; it calls moths, lanterns, wardens and drifters.")
m(50,"choomstar","Choomstar","gauntlet","cubo","any",["49","45:gold","28:gold","37:gold","40:gold","48:gold"],{"kind":"gauntlet","stages":4,"repairs":2},"health",1,30,60,[{"boss":"black_knight"},{"boss":"section_chief"},{"boss":"great_horned"},{"boss":"bailiff"}],"Every boss, one after another. Shown only once the Interdictor falls.",secret=True,row=7)

ids = {x["n"]: x for x in M}
def tier(x): return x.get("row", len(x["requires"]))
problems = []
assert len(M) == 50, len(M)
for x in M:
    for r in x["requires"]:
        k = int(r.split(":")[0])
        if tier(ids[k]) >= tier(x): problems.append(f'{x["n"]} requires {k}, not in a lower tier')
assert not problems, problems
for x in M:
    x["requires"] = [{"mission": ids[int(r.split(":")[0])]["id"], "medal": (r.split(":")[1] if ":" in r else "bronze")} for r in x["requires"]]
doc = {"_about": "The missions (draft, journal: Missions). A mission's tier is the number of its requirements (after Vagrancy's road). A requirement names an earlier mission and the medal it needs: bronze (any clear), silver or gold. Medal thresholds are in the measure's unit: seconds for time (lower is better), percent of health left for health, relics carried for relics. Every number is a starting point for playtest.", "missions": [{k: v for k, v in x.items() if k != "n"} for x in M]}
out = Path(sys.argv[1] if len(sys.argv) > 1 else "data/missions.json")
out.write_text(json.dumps(doc, indent=1, ensure_ascii=False) + "\n")
from collections import Counter
print("missions", len(M), "tiers", sorted(Counter(tier(ids[i]) for i in ids).items()), "kinds", sorted(Counter(x["kind"] for x in M).items()))
