/*FILE			:	main.rs
*PROJECT		:	Pointer
*PROGRAMER		:	Doosan Beak
*FIRST VERSION	:	2020-04-17
*DESCRIPTION    :   Methods in this file support main and the main tests clear screen, display video memory,
					set cursor position.
*/

fn main() {
    let mut video=VideoSim{
        current_column:0,
        current_row:0,
        video_memory:&mut[' ';MAX_COLS*MAX_ROWS]
    };
    video.clear_screen();
    video.output_string(&std::string::String::from("top left corner"));
    video.display_video_memory();


    video.clear_screen();
    if video.set_cursor_position(10,5){
        video.output_string(&std::string::String::from("here's some text in the middle of the screen"));
        video.display_video_memory();
    }
    


    video.clear_screen();
    if video.set_cursor_position(23,39){
        video.output_string(&std::string::String::from("here's text that will scroll the screen!"));
        video.display_video_memory();
    }
   
    video.clear_screen();
    if video.set_cursor_position(40,39){
        video.output_string(&std::string::String::from("here's text that will scroll the screen!"));
        video.display_video_memory();
    } else{
        println!("Please check you input, x should be between 0 and 40, y should be between 0 and 24");
    } 

}

const MAX_COLS: usize = 40;
const MAX_ROWS: usize = 24;


/*
*FUNCTION		: VideoSim
*PARAMETERS		: no parameter
*RETURNS		: no return
*DESCRIPTION	: This constructor set currnet row and currunt column to 0, and clear memory.
*/
//#[derive(Debug)]
struct VideoSim<'a> {
    video_memory:&'a mut [char;MAX_COLS*MAX_ROWS],
    current_row:usize,
    current_column:usize
}

impl VideoSim<'_> {
    /*
    *FUNCTION		: display_video_memory
    *PARAMETERS		: no parameter
    *RETURNS		: no return
    *DESCRIPTION	: This method display strings within the memory 
    */
    fn display_video_memory(&self){
        println!("Video memory holds");
        print!("   ");
        for n in 0..MAX_COLS {
            if (n % 10) == 0 {
                print!("{}", n / 10);
            } else{
                print!(" ");
            } 
        }
        println!();

        print!("   ");
        for n in 0..MAX_COLS {
            print!("{}", n % 10);
        }
        println!();

        
        for n in 0..MAX_ROWS{
            print!("{:02} ",n);
            for i in 0..MAX_COLS {
                print!("{}",self.video_memory[n*MAX_COLS+i]);
            }
            println!();
        }
        
    }
    /*
    *FUNCTION		: clear_screen
    *PARAMETERS		: no parameter
    *RETURNS		: no return
    *DESCRIPTION	: This method clear screen by filling it with empty spaces
    */
    fn clear_screen(&mut self){
        for n in 0..MAX_COLS*MAX_ROWS {
            self.video_memory[n]=' ';
        }
    }
    /*
    *FUNCTION		: set_cursor_position
    *PARAMETERS		: int x : row     int y : column
    *RETURNS		: return 0 if row and column are not in range. return 1 if row and column are in range.
    *DESCRIPTION	: This method update row and column
    */
    fn set_cursor_position(&mut self,x:usize,y:usize)->bool{
        let mut ret=false;
        if x>=0 && x<MAX_ROWS && y>=0 && y< MAX_COLS{
            self.current_row = x;
            self.current_column = y;
            println!("set the row={}, and cloumn={}",self.current_row,self.current_column);
            ret=true;
        }
        return ret;
    }
    /*
    *FUNCTION		: output_string
    *PARAMETERS		: char* string : the string user wants to write
    *RETURNS		: no return
    *DESCRIPTION	: This method find updated position and write string to there
    */
    fn output_string(&mut self, message:&String){
        let mut vid_off_set = self.current_row * MAX_COLS + self.current_column;

        for c in message.chars(){

            if vid_off_set >= MAX_ROWS * MAX_COLS{
                println!("current row {}, vid off set {}",self.current_row,vid_off_set);
                
                self.scroll_sreen();
                vid_off_set = (MAX_ROWS-1) * MAX_COLS;
                self.video_memory[vid_off_set]=c;
                vid_off_set+=1;
                
            } else {
                self.video_memory[vid_off_set]=c;
                vid_off_set+=1;
            }
            

        }
    }
    /*
    *FUNCTION		: scroll_sreen
    *PARAMETERS		: no parameter
    *RETURNS		: no return
    *DESCRIPTION	: This method copy second row - last row to first row - second last row and clean last row
    */
    fn scroll_sreen(&mut self){
        for r in 1..MAX_ROWS{
            for c in 0..MAX_COLS{
                self.video_memory[(r-1)*MAX_COLS+c]=self.video_memory[r*MAX_COLS+c];
            }
        }
        for c in 0..MAX_COLS{
            self.video_memory[(MAX_ROWS-1)*MAX_COLS+c]=' ';
        }

    }
}