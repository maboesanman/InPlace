var searchIndex = JSON.parse('{\
"in_place":{"doc":"A trait formulation of the Entry API, to make working with …","t":[0,0,0,0,0,3,6,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,16,16,4,8,4,8,8,16,16,8,8,13,13,16,8,8,8,16,13,13,16,8,16,16,11,11,11,11,11,11,11,11,11,10,11,10,11,10,11,10,10,11,11,11,10,11,11,11,10,11,10,10,11,10,11,11,10,11,11,10,10,11,11,11,11,10,11,11,11,11,11,11,11,11,11,12,12,12,12,12,8,8,8,8,8,16,16,16,16,16,10,11,11,10,10,10,10,11,11,11,11,11,11,10],"n":["dummy_collections","entry","get_entry","implementations","hash_map","DummyHashMap","DummyHashMapEntry","DummyHashMapOccupiedEntry","DummyHashMapVacantEntry","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","from","from","from","get_entry","get_entry_with_key","get_key","get_pair","get_pair_mut","get_value","get_value_mut","into","into","into","into_key","into_pair","into_value_mut","occupy","recover_removed_entry","remove","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","vacate","BorrowedKey","Collection","Entry","EntryRemovableOccupiedEntry","EntryWithSearchKey","InsertableOccupiedEntry","IntoCollectionMut","Key","Key","KeyedOccupiedEntry","KeyedVacantEntry","Occupied","Occupied","Occupied","OccupiedEntry","OccupiedEntryKeyMut","RemovableOccupiedEntry","Removed","Vacant","Vacant","Vacant","VacantEntry","Value","Value","borrow","borrow","borrow_mut","borrow_mut","from","from","from","from_occupied","from_vacant","get_key","get_key","get_pair","get_pair","get_pair_mut","get_pair_mut","get_value","get_value_mut","insert","insert","insert_into_entry","insert_new","insert_new","into","into","into_collection_mut","into_collection_mut","into_key","into_pair","into_pair","into_value_mut","is_occupied","is_vacant","occupy","occupy","occupy_new","recover_removed_entry","remove","remove_entry","remove_value","remove_value","remove_with_key","replace_key","replace_value","replace_value","try_from","try_from","try_into","try_into","type_id","type_id","vacate","0","0","0","0","1","GetEntryByIndex","GetEntryByKey","GetEntryFromKey","GetFirstEntry","GetLastEntry","Occupied","Occupied","Occupied","Occupied","Vacant","get_entry","get_entry_from_key","get_entry_from_key","get_entry_with_key","get_first_occupied","get_last_occupied","get_occupied","insert_into_entry","insert_into_entry","occupy","occupy","remove_entry","remove_entry","vacate"],"q":["in_place","","","","in_place::dummy_collections","in_place::dummy_collections::hash_map","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","in_place::entry","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","in_place::entry::Entry","","in_place::entry::EntryWithSearchKey","","","in_place::get_entry","","","","","","","","","","","","","","","","","","","","","","",""],"d":["This module contains stubbed out examples of what this API …","This module allows you to manipulate entries you have …","This module allows you to aquire entries from collections.","This module contains implementations of traits on …","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","An entry which is either Occupied or Vacant.","A trait to represent recovering to an entry on removal.","","A trait to represent inserting a new entry on top of an …","","the type of the keys in the collection","","A trait to represent an occupied entry of a collection …","","","","The type of OccupiedEntry we convert to when inserting.","A trait to represent an occupied entry of a collection.","A trait to extend OccupiedEntry, allowing the user to …","A trait to represent an OccupiedEntry that can be removed …","The type returned when removing the OccupiedEntry","","","","A trait to represent a vacant entry of a collection.","the type of the values in the collection","The type of values in the collection.","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Get a reference to the key an item will be inserted with.","","get the key value pair, immutably borrowed","","get the key value pair. the key is immutably borrowed, and …","","get the value, immutably borrowed.","get the value, mutably borrowed.","insert the <code>val</code> using the owned key.","insert the <code>val</code> using the owned key.","","insert a new element at this position, shifting the others …","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","Consume self and return the contained key.","convert the entry into key value pair. the key is …","","convert the entry into a mutable to the value.","","","insert the <code>val</code> using the owned key, returning the occupied …","set the value, returning the OccupiedEntry, and the …","","","remove this entry from the collection, converting the entry","","remove this entry from the collection, consuming the entry …","remove this entry from the collection, consuming the entry …","","replace key with another one.","replace the value in the entry, returning the old value.","replace the value in the entry, returning the old value.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,1,2,3,1,2,3,1,2,3,1,1,3,2,2,2,2,1,2,3,3,2,2,3,2,2,1,2,3,1,2,3,1,2,3,1,4,5,0,0,0,0,0,4,6,0,0,7,8,9,0,0,0,10,7,8,11,0,12,9,7,8,7,8,7,7,8,7,7,6,7,4,7,4,7,12,12,9,9,7,13,7,7,8,5,7,6,4,7,12,7,7,9,7,7,11,10,7,10,10,7,14,12,12,7,8,7,8,7,8,7,15,16,17,18,17,0,0,0,0,0,19,20,21,22,19,23,19,19,19,20,21,22,19,19,19,19,23,23,23],"f":[null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],[[]],[[["",0],["",0]],["entry",4]],[[["",0]],["entrywithsearchkey",4]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["dummyhashmapentry",6]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0],["",0]]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["entrywithsearchkey",4]]],[[]],[[]],[[]],[[]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]],["result",4]],[[["",0]]],[[["",0]],["result",4]],[[["",0]]],[[["",0]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[]],[[["",0]],["bool",0]],[[["",0]],["bool",0]],[[]],[[]],[[]],[[],["entry",4]],[[]],[[]],[[]],[[]],[[]],[[["",0]],["result",4]],[[["",0]]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0],["",0]],["entry",4]],[[["",0]],["entry",4]],[[["",0]],["entry",4]],[[["",0]],["entrywithsearchkey",4]],[[["",0]],["option",4]],[[["",0]],["option",4]],[[["",0],["usize",0]],["option",4]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]]],"p":[[3,"DummyHashMap"],[3,"DummyHashMapOccupiedEntry"],[3,"DummyHashMapVacantEntry"],[8,"KeyedOccupiedEntry"],[8,"IntoCollectionMut"],[8,"KeyedVacantEntry"],[4,"Entry"],[4,"EntryWithSearchKey"],[8,"VacantEntry"],[8,"RemovableOccupiedEntry"],[8,"EntryRemovableOccupiedEntry"],[8,"OccupiedEntry"],[8,"InsertableOccupiedEntry"],[8,"OccupiedEntryKeyMut"],[13,"Occupied"],[13,"Vacant"],[13,"Occupied"],[13,"Vacant"],[8,"GetEntryFromKey"],[8,"GetFirstEntry"],[8,"GetLastEntry"],[8,"GetEntryByIndex"],[8,"GetEntryByKey"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};