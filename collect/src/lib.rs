fn swap(arr: &mut [i32],i:usize,j:usize){
    let saving=arr[i];
    arr[i]=arr[j];
    arr[j]=saving;
}

pub fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len()-1{
        for j in i+1..arr.len(){
            if arr[i]>arr[j]{
                swap(arr,i,j);
            }
        }
    }
}