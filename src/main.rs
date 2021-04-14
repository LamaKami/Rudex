mod indexing;

use crate::indexing::index;
use crate::indexing::postlist;

use std::collections::HashMap;
use postlist::Posting;
//TODO Multiprocessing, Stoppwoerter, Tokens, NGramme, update Index
/*
1. Lade Index wenn moeglich, erstelle sonst einen neuen leeren
2. Loop ueber alle Dokumente drueber
    2.1 Falls Dokuement schon vorhanden (teste ueber UUID) skip
        2.1.2 Eigene Date mit allen Doks ist glaube ich sinnvoll -> DB
    2.2 Dokument nicht vorhanden -> Loop ueber jedes Wort
        2.2.1 Lemmatisiere oder Stemme Wort etc. (Spaeter)
        2.2.2 Schau ob Wort im Index ist
            Ja: Erzeuge ein Posting und fuege es an der richtigen Stelle in der Postlist hinzu
                Spaeter noch update Skippointer
            Nein: Erzeuge ein Posting, Erstelle neue Postlist und fuege es dann hinzu
3. Speichere Index


Durchfuehrung eines Dokuments:
    new HashMap<wort,Posting>
    for word in document:
        if word in HashMap:
            update Posting
        else:
            new Posting
            add Posting to HashMap
    for element in HashMap:
        if elment.word in index:
            add Posting zu existierenden PostingList
        else:
            create new PostingList mit elment.word als Key

Frage: Ist die Wahl der UUID sinnvoll
 */

fn main() {

    index::create_index("data/test/");

    //Postings muessen noch sortiert werden spaeter nach documentId



    let index: HashMap<String,Vec<Posting>> = index::load_index("data/index/indexlarge.txt");

    println!("{:?}", index);

    /*
    let index = index::create_index("data/pos/");
    
    let result = searcher::search(&index, "german dog blood transfused");

    let sorted_result = sorting::bubble_sort(&result);
    for (i, doc) in sorted_result.iter().enumerate() {
        if i > 10{
            break;
        }
        println!("{} {}", doc, result.get(doc).unwrap());
    }
    */

}



//Index builder, nur std libaries
// text als variable dann dateien einlesen txt, dann json

//Baue Hash Map die ein query wort als Key hat und der value die documente sind?
//Postlisten : t1 -> [document,[Gewichte],[positionen], list length, [skip pointer]]
//list length:Added to the first entry of the posting list of a term t, Stores the length of the posting list, Equals the number of documents containing t (document frequency df(t, D))
//skip pointer gibt die id des n-ten Dokuments an 