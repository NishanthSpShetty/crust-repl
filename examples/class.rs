 /*************************************************************************
 * This file was generated by CRUST by an automated semantics preserving
 * translation from C/C++ to Rust
 * CRUST isn't perfect and may require manual editing
 * Check warnings and errors and refer to the official Rust Documentation
 ************************************************************************/
 struct A { a : i32 , b : i32 , } 

/**Method declarations are wrapped inside the impl block 
 * Which implements the corresponding structure
 **/
 impl A {
 
/** Rust structures do not support constructors
 * Please handle them with static builder functions
 * >>>>>>>> A ( ) { a = 5 ; b = 6 ; } 
 **/
 fn getfloat ( &self ) -> f32 { 
/** Crust tries to identify return statement and replace with rust equivalent
 * shorthand notation. If error found in this line, Please replace shorthand notation 
 * with return statement 
 **/
 1.23 } pub fn getInt ( &self , a : i32 ) -> i32 { 
/** Crust tries to identify return statement and replace with rust equivalent
 * shorthand notation. If error found in this line, Please replace shorthand notation 
 * with return statement 
 **/
 a } }
 struct B { aa : i32 , bb : i32 } 

/**Method declarations are wrapped inside the impl block 
 * Which implements the corresponding structure
 **/
 impl B {
 }
 struct Address { id : i32 , name : char , [ : char ] : char postal : char , pin : i32 , } 

/**Method declarations are wrapped inside the impl block 
 * Which implements the corresponding structure
 **/
 impl Address {
 }
 fn main ( ) { 
/** Declaration of a structure should be completed with initialization of it's fields
 * It should be in the following format
 * let variable:struct_name = struct_name { member1:value1, member2:value2,..}
 */ let b = B { aa : 0i32 , bb : 0i32 , }; 
/** Declaration of a structure should be completed with initialization of it's fields
 * It should be in the following format
 * let variable:struct_name = struct_name { member1:value1, member2:value2,..}
 */ let a = A { a : 0i32 , b : 0i32 , }; a . getInt ( ) ; a . getfloat ( ) ; 
/** Declaration of a structure should be completed with initialization of it's fields
 * It should be in the following format
 * let variable:struct_name = struct_name { member1:value1, member2:value2,..}
 */ let add = Address { id : 0i32 , name : '_' , [ : '_' , ] : '_' , postal : '_' , pin : 0i32 , }; //do some thing else
 }