use std::rc::Rc;

use crate::logic::util::{client, sn_client};
use slint::{VecModel, StandardListViewItem};
// use ::chrono::Local;
use tiberius::time::chrono;
// use uuid::Uuid;

pub async fn sn_work(s: String) -> (Rc<VecModel<slint::ModelRc<StandardListViewItem>>>,usize) {
    let mut row_data = Vec::new();
    let mut client = sn_client().await;
    let query_ty = format!("where SN = '{}'", s);
    // let testtype = "SN,ProductBill,TestType,Result,Ith,Pf,Vop,Im,Rs,Sen,Res,ICC,Idark,Vbr,IXtalk,Kink,TestDate";
    let testtype_12 = "SN,ProductBill,TestType,Result,Ith,Po,Vf,Im,Rs,Sen,Res,ICC,Idark,Vbr,Xtalk,Kink_I,TestDate";
    // let stream  = client.query(format!("
    // SELECT {3} FROM {1}MAC_10GBOSADATA  {0} UNION all SELECT {4} FROM {2}MAC_f07959df8122  {0}
    // UNION all SELECT {4} FROM {2}MAC_00e04c686dd1  {0}  UNION all SELECT {4} FROM {2}MAC_00e04c3105fb  {0}
    // UNION all SELECT {4} FROM {2}MAC_70e400a2c0d4  {0}  UNION all SELECT {4} FROM {2}MAC_00262da5e778  {0}
    // UNION all SELECT {4} FROM {2}MAC_408d5cb2d04a  {0}  UNION all SELECT {4} FROM {2}MAC_408d5cb712a3  {0}
    // UNION all SELECT {4} FROM {2}MAC_408d5cb72176  {0}  UNION all SELECT {4} FROM {2}MAC_10634b007c4b  {0}
    // UNION all SELECT {4} FROM {2}MAC_10634b0966f0  {0}  UNION all SELECT {4} FROM {2}MAC_08626683922a  {0}
    // UNION all SELECT {4} FROM {2}MAC_c8e7d8e187a7  {0}  UNION all SELECT {4} FROM {2}MAC_f0b42937e924  {0}
    // UNION all SELECT {4} FROM {2}MAC_fcaa14afca45  {0}  UNION all SELECT {4} FROM {2}MAC_F07959DF968E  {0}
    // UNION all SELECT {4} FROM {2}MAC_f07959df9218  {0}  UNION all SELECT {4} FROM {2}MAC_f07959df9694  {0}
    // UNION all SELECT {4} FROM {2}MAC_f07959dfa742  {0}  UNION all SELECT {4} FROM {2}MAC_fcaa14db2983  {0}  ",
    let stream  = client.query(format!("SELECT {1} FROM MAC_10GBOSADATA  {0}",query_ty,testtype_12),&[&1i32]).await.unwrap();
    let rowsets = stream.into_results().await.unwrap();
    for i in 0..rowsets.len() {
        let rows = rowsets.get(i).unwrap();
        for row in rows {
            let items = Rc::new(VecModel::default());
            let sn = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(0).unwrap_or("?"))));
            let workid = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(1).unwrap_or("?"))));
            let pn = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(2).unwrap_or("?"))));
            let result = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(3).unwrap_or("?"))));
            let ith = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(4).unwrap_or("?"))));
            let pf = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(5).unwrap_or("?"))));
            let vop = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(6).unwrap_or("?"))));
            let im = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(7).unwrap_or("?"))));
            let rs = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(8).unwrap_or("?"))));
            let sen = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(9).unwrap_or("?"))));
            let res = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(10).unwrap_or("?"))));
            let icc = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(11).unwrap_or("?"))));
            let idark = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(12).unwrap_or("?"))));
            let vbr = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(13).unwrap_or("?"))));
            let ixtalk = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(14).unwrap_or("?"))));
            let kink = StandardListViewItem::from(slint::format!("{}", (row.get::<&str, _>(15).unwrap_or("?"))));
            let datatime = StandardListViewItem::from(slint::format!(
                "{}",
                (row.get::<chrono::NaiveDateTime, _>(16)
                    .unwrap()
                    .format("%Y/%m/%d %H:%M:%S"))
            ));
            items.push(sn);
            items.push(workid);
            items.push(pn);
            items.push(result);
            items.push(ith);
            items.push(pf);
            items.push(vop);
            items.push(im);
            items.push(rs);
            items.push(sen);
            items.push(res);
            items.push(icc);
            items.push(idark);
            items.push(vbr);
            items.push(ixtalk);
            items.push(kink);
            items.push(datatime);
            row_data.push(items.into());
        }
    }
    let m = row_data.len();
    let data = Rc::new(VecModel::from(row_data));
    (data, m)
}
// // // pub async fn box_none_work(d1: String, d2: String) -> (Vec<Vec<String>>, usize) {
// // //     let mut client = client().await;
// // //     let query_ty = format!(
// // //         "where CreateTime Between '{}' and '{}' ORDER BY [CreateTime] DESC OFFSET 0 ROWS ",
// // //         d1, d2
// // //     );
// // //     let testtype_none = "Pack_no,Sn,PN,WorkOrder,Creator,CreateTime";
// // //     let stream = client
// // //         .query(
// // //             format!(
// // //                 "select {0} from MaterialPackSn {1} ",
// // //                 testtype_none, query_ty
// // //             ),
// // //             &[&1i32],
// // //         )
// // //         .await
// // //         .unwrap();
// // //     let rowsets = stream.into_results().await.unwrap();
// // //     let mut rv: Vec<Vec<String>> = vec![];
// // //     //let no:Vec<String> = vec![];
// // //     for i in 0..rowsets.len() {
// // //         let rows = rowsets.get(i).unwrap();
// // //         for row in rows {
// // //             let mut v: Vec<String> = vec![];
// // //             v.push(row.get::<&str, _>(0).unwrap().to_string());
// // //             v.push(row.get::<&str, _>(1).unwrap().to_string());
// // //             v.push(row.get::<&str, _>(2).unwrap().to_string());
// // //             v.push(row.get::<&str, _>(3).unwrap().to_string());
// // //             v.push(row.get::<&str, _>(4).unwrap().to_string());
// // //             v.push(row.get::<chrono::NaiveDateTime, _>(5).unwrap().to_string());
// // //             rv.insert(i, v)
// // //         }
// // //     }
// // //     let quantity = rv.len();
// // //     (rv, quantity)
// // // }



pub async fn box_work(s: String) -> (Rc<VecModel<slint::ModelRc<StandardListViewItem>>>,usize) {
    let mut row_data: Vec<slint::ModelRc<StandardListViewItem>> = vec![];
    let mut client = client().await;
    let query_ty = format!(
        "where Pack_no = '{}' ORDER BY [CreateTime] DESC OFFSET 0 ROWS ",
        s
    );
    let testtype_none = "Pack_no,Sn,PN,WorkOrder,Creator,CreateTime";
    let stream = client
        .query(
            format!(
                "select {0} from MaterialPackSn {1} ",
                testtype_none, query_ty
            ),
            &[&1i32],
        )
        .await
        .unwrap();
    let rowsets = stream.into_results().await.unwrap();
    for i in 0..rowsets.len() {
        let rows = rowsets.get(i).unwrap();
        for row in rows {
            let items = Rc::new(VecModel::default());
            let box_no = StandardListViewItem::from(slint::format!(
                "{}",
                (row.get::<&str, _>(0).unwrap_or("?"))
            ));
            let sn = StandardListViewItem::from(slint::format!(
                "{}",
                (row.get::<&str, _>(1).unwrap_or("?"))
            ));
            let pn = StandardListViewItem::from(slint::format!(
                "{}",
                (row.get::<&str, _>(2).unwrap_or("?"))
            ));
            let order = StandardListViewItem::from(slint::format!(
                "{}",
                (row.get::<&str, _>(3).unwrap_or("?"))
            ));
            let worderid = StandardListViewItem::from(slint::format!(
                "{}",
                (row.get::<&str, _>(3).unwrap_or("?"))
            ));
            let datatime = StandardListViewItem::from(slint::format!(
                "{}",
                (row.get::<chrono::NaiveDateTime, _>(5)
                    .unwrap()
                    .format("%Y/%m/%d %H:%M:%S"))
            ));
            items.push(box_no);
            items.push(sn);
            items.push(pn);
            items.push(order);
            items.push(worderid);
            items.push(datatime);
            row_data.push(items.into());
        }
    }
    let quantity = row_data.len();
    let data = Rc::new(VecModel::from(row_data));
    (data,quantity)
}
// pub async fn carton_none_work(d1: String, d2: String) -> (Vec<Vec<String>>, i32) {
//     let mut client = client().await;
//     let mut client_1 = client_1().await;
//     let mut client_2 = client_2().await;
//     let query_ty = format!(
//         "where CreateTime Between '{}' and '{}'group by CartonNo ",
//         d1, d2
//     );
//     let query_sum = format!("where CreateTime Between '{}' and '{}'", d1, d2);
//     let testtype_carton_none = "CartonNo,PN,Creator,CreateTime";
//     let stream  = client.query(format!("SELECT {0} FROM packing_carton WHERE CartonID IN (select min(CartonID) from packing_carton {1} )",testtype_carton_none,query_ty),&[&1i32]).await.unwrap();
//     let stream_1 = client_1
//         .query(
//             format!(
//                 "SELECT SUM(Packing_no_BOX) FROM packing_carton {}",
//                 query_sum
//             ),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let rowsets = stream.into_results().await.unwrap();
//     let rowsets_1 = stream_1.into_row().await.unwrap().unwrap();
//     let sum = rowsets_1.get::<i32, _>(0);
//     let sum = match sum {
//         Some(sun) => sun,
//         None => 0,
//     };

//     let mut rv: Vec<Vec<String>> = vec![];
//     for i in 0..rowsets.len() {
//         let rows = rowsets.get(i).unwrap();
//         for row in rows {
//             let car = row.get::<&str, _>(0).unwrap().to_string();
//             let stream_2 = client_2
//                 .query(
//                     format!(
//                         "SELECT SUM(Packing_no_BOX) FROM packing_carton WHERE CartonNo = '{}'",
//                         car
//                     ),
//                     &[&1i32],
//                 )
//                 .await
//                 .unwrap();
//             let rowsets_2 = stream_2.into_row().await.unwrap().unwrap();
//             let mut v: Vec<String> = vec![];
//             v.push(row.get::<&str, _>(0).unwrap().to_string());
//             v.push(row.get::<&str, _>(1).unwrap().to_string());
//             v.push(row.get::<&str, _>(2).unwrap().to_string());
//             v.push(rowsets_2.get::<i32, _>(0).unwrap().to_string());
//             v.push(row.get::<chrono::NaiveDateTime, _>(3).unwrap().to_string());
//             rv.insert(i, v)
//         }
//     }
//     (rv, sum)
// }
// pub async fn carton_work(s: String) -> Rc<VecModel<slint::ModelRc<StandardListViewItem>>> {
//     let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());
//     let mut client = client().await;
//     let mut client_1 = client_1().await;
//     let mut client_2 = client_2().await;
//     let car_no = s.clone();
//     let stream = client.query(format!("select a.sn from MaterialPackSn a inner join  packing_carton b on a.Pack_no=b.Packing_no where b.cartonno='{}' order by b.CreateTime  desc,b.Packing_no  desc,a.Pack_no asc",car_no), &[&1i32]).await.unwrap();
//     let rowsets = stream.into_results().await.unwrap();
//     //let mut box_m  = vec![];
//     // let mut rv: Vec<Vec<String>> = vec![];
//     let rows = rowsets.get(0).unwrap();
//     for (_i, row) in rows.iter().enumerate() {
//         let sn_s = row.get::<&str, _>(0).unwrap().to_string();
//         let stream_1 = client_1
//             .query(
//                 format!("select Pack_no from MaterialPackSn where Sn ='{}'", sn_s),
//                 &[&1i32],
//             )
//             .await
//             .unwrap();
//         let rowsets_1 = stream_1.into_row().await.unwrap().unwrap();
//         let box_s = rowsets_1.get::<&str, _>(0).unwrap().to_string();

//         let testtype_22 = "SN,Ith,SE,Po,Vf,Im,Sen";
//         let stream_2  = client_2.query(format!("select {} from PACK_TESTDATA_HYD WHERE SN ='{}' and pnoptionid='-100' and  testDataType= '初测'",testtype_22,sn_s),&[&1i32]).await.unwrap();
//         let row_2 = stream_2.into_row().await.unwrap().unwrap();
//         let car = s.clone();
//         let items = Rc::new(VecModel::default());

//         let sn = slint::format!(
//             "{}",
//             (row_2.get::<&str, _>(0).unwrap_or("?"))
//         ));
//         let ith = slint::format!(
//             "{}",
//             (row_2.get::<&str, _>(1).unwrap_or("?"))
//         ));
//         let se = slint::format!(
//             "{}",
//             (row_2.get::<&str, _>(2).unwrap_or("?"))
//         ));
//         let po = row_2.get::<&str, _>(3).unwrap().to_string();
//         let s_po = po.parse::<f64>().unwrap() / 1000.0;
//         let f_po = format!("{:.2}", s_po);
//         let po = slint::format!("{}", f_po));
//         let vf = slint::format!(
//             "{}",
//             (row_2.get::<&str, _>(4).unwrap_or("?"))
//         ));
//         let im = slint::format!(
//             "{}",
//             (row_2.get::<&str, _>(5).unwrap_or("?"))
//         ));
//         let sen = slint::format!(
//             "{}",
//             (row_2.get::<&str, _>(6).unwrap_or("?"))
//         ));
//         items.push(sn);
//         items.push(StandardListViewItem::from(slint::format!("25℃")));
//         items.push(ith);
//         items.push(se);
//         items.push(po);
//         items.push(vf);
//         items.push(im);
//         items.push(sen);
//         items.push(StandardListViewItem::from(slint::format!("{}", box_s)));
//         items.push(StandardListViewItem::from(slint::format!("{}", car)));
//         row_data.push(items.into());
//     }
//     // let quantity = rv.len();
//     row_data
// }

// pub async fn multiple_sn_work(vec_sn: Vec<String>) -> Vec<Vec<String>> {
//     let mut client = sn_client().await;
//     let mut rv: Vec<Vec<String>> = vec![];
//     for s in vec_sn {
//         let query_ty = format!("where SN = '{}'", s);
//         let testtype = "SN,ProductBill,TestType,Result,Ith,Pf,Vop,Im,Rs,Sen,Res,ICC,Idark,Vbr,IXtalk,Kink,TestDate";
//         let testtype_12 = "SN,ProductBill,TestType,Result,Ith,Po,Vf,Im,Rs,Sen,Res,ICC,Idark,Vbr,Xtalk,Kink_I,TestDate";
//         let stream  = client.query(format!("
//         SELECT {3} FROM {1}MAC_10GBOSADATA  {0} UNION all SELECT {4} FROM {2}MAC_f07959df8122  {0}
//         UNION all SELECT {4} FROM {2}MAC_00e04c686dd1  {0}  UNION all SELECT {4} FROM {2}MAC_00e04c3105fb  {0}
//         UNION all SELECT {4} FROM {2}MAC_70e400a2c0d4  {0}  UNION all SELECT {4} FROM {2}MAC_00262da5e778  {0}
//         UNION all SELECT {4} FROM {2}MAC_408d5cb2d04a  {0}  UNION all SELECT {4} FROM {2}MAC_408d5cb712a3  {0}
//         UNION all SELECT {4} FROM {2}MAC_408d5cb72176  {0}  UNION all SELECT {4} FROM {2}MAC_10634b007c4b  {0}
//         UNION all SELECT {4} FROM {2}MAC_10634b0966f0  {0}  UNION all SELECT {4} FROM {2}MAC_08626683922a  {0}
//         UNION all SELECT {4} FROM {2}MAC_c8e7d8e187a7  {0}  UNION all SELECT {4} FROM {2}MAC_f0b42937e924  {0}
//         UNION all SELECT {4} FROM {2}MAC_fcaa14afca45  {0}  UNION all SELECT {4} FROM {2}MAC_F07959DF968E  {0}
//         UNION all SELECT {4} FROM {2}MAC_f07959df9218  {0}  UNION all SELECT {4} FROM {2}MAC_f07959df9694  {0}
//         UNION all SELECT {4} FROM {2}MAC_f07959dfa742  {0}  UNION all SELECT {4} FROM {2}MAC_fcaa14db2983  {0}  ",
//         query_ty,"BOSAautotest_Data.dbo.","BOSAautotestDB.dbo.",testtype,testtype_12),&[&1i32]).await.unwrap();
//         let rowsets = stream.into_results().await.unwrap();
//         for i in 0..rowsets.len() {
//             let rows = rowsets.get(i).unwrap();
//             for row in rows {
//                 let mut v: Vec<String> = vec![];
//                 v.push(row.get::<&str, _>(0).unwrap().to_string());
//                 v.push(row.get::<&str, _>(1).unwrap().to_string());
//                 v.push(row.get::<&str, _>(2).unwrap().to_string());
//                 v.push(row.get::<&str, _>(3).unwrap().to_string());
//                 v.push(row.get::<&str, _>(4).unwrap().to_string());
//                 v.push(row.get::<&str, _>(5).unwrap().to_string());
//                 v.push(row.get::<&str, _>(6).unwrap().to_string());
//                 v.push(row.get::<&str, _>(7).unwrap().to_string());
//                 v.push(row.get::<&str, _>(8).unwrap().to_string());
//                 v.push(row.get::<&str, _>(9).unwrap().to_string());
//                 v.push(row.get::<&str, _>(10).unwrap().to_string());
//                 v.push(row.get::<&str, _>(11).unwrap().to_string());
//                 v.push(row.get::<&str, _>(12).unwrap().to_string());
//                 v.push(row.get::<&str, _>(13).unwrap().to_string());
//                 v.push(row.get::<&str, _>(14).unwrap().to_string());
//                 v.push(row.get::<&str, _>(15).unwrap().to_string());
//                 v.push(
//                     row.get::<chrono::NaiveDateTime, _>(16)
//                         .unwrap()
//                         .format("%Y/%m/%d %H:%M:%S")
//                         .to_string(),
//                 );
//                 rv.insert(i, v)
//             }
//         }
//     }
//     rv
// }
// pub async fn multiple_carton_work(vec_carton: Vec<String>) -> (Vec<Vec<String>>, usize) {
//     let mut client = client().await;
//     let mut client_1 = client_1().await;
//     let mut client_2 = client_2().await;
//     //let box_m  = vec![];
//     let mut rv: Vec<Vec<String>> = vec![];
//     for s in vec_carton {
//         let car_no = s.clone();
//         let stream = client.query(format!("select a.sn from MaterialPackSn a inner join  packing_carton b on a.Pack_no=b.Packing_no where b.cartonno='{}' order by b.CreateTime  desc,b.Packing_no  desc,a.Pack_no asc",car_no), &[&1i32]).await.unwrap();
//         let rowsets = stream.into_results().await.unwrap();
//         let rows = rowsets.get(0).unwrap();
//         for (i, row) in rows.iter().enumerate() {
//             let sn_s = row.get::<&str, _>(0).unwrap().to_string();
//             let stream_1 = client_1
//                 .query(
//                     format!("select Pack_no from MaterialPackSn where Sn ='{}'", sn_s),
//                     &[&1i32],
//                 )
//                 .await
//                 .unwrap();
//             let rowsets_1 = stream_1.into_row().await.unwrap().unwrap();
//             let box_s = rowsets_1.get::<&str, _>(0).unwrap().to_string();

//             let testtype_22 = "SN,Ith,SE,Po,Vf,Im,Sen";
//             let stream_2  = client_2.query(format!("select {} from PACK_TESTDATA_HYD WHERE SN ='{}' and pnoptionid='-100' and  testDataType= '初测'",testtype_22,sn_s),&[&1i32]).await.unwrap();
//             let row_2 = stream_2.into_row().await.unwrap().unwrap();
//             let car = s.clone();
//             let mut v: Vec<String> = vec![];
//             v.push(row_2.get::<&str, _>(0).unwrap().to_string());
//             v.push("25℃".to_string());
//             v.push(row_2.get::<&str, _>(1).unwrap().to_string());
//             v.push(row_2.get::<&str, _>(2).unwrap().to_string());
//             let po = row_2.get::<&str, _>(3).unwrap().to_string();
//             let s_po = po.parse::<f64>().unwrap() / 1000.0;
//             let f_po = format!("{:.2}", s_po);
//             v.push(f_po.to_string());
//             v.push(row_2.get::<&str, _>(4).unwrap().to_string());
//             v.push(row_2.get::<&str, _>(5).unwrap().to_string());
//             v.push(row_2.get::<&str, _>(6).unwrap().to_string());
//             v.push(box_s);
//             v.push(car);
//             let v_1 = v.clone();
//             rv.insert(i, v_1);
//         }
//     }
//     let quantity = rv.len();
//     (rv, quantity)
// }

// pub async fn workerid_work(s: String, d1: String, d2: String) -> (Vec<Vec<String>>, usize) {
//     let mut client = sn_client().await;
//     let query_ty = format!(
//         "where ProductBill ='{}' and Result = 'OK' and TestDate Between '{}' and '{}'",
//         s, d1, d2
//     );
//     let testtype = "SN,TestType,Result,Ith,Pf,Im,Sen,Res,ICC,Idark,Kink,TestDate";
//     let testtype_12 = "SN,TestType,Result,Ith,Po,Im,Sen,Res,ICC,Idark,Kink_I,TestDate";
//     let stream  = client.query(format!("
//     SELECT {3} FROM {1}MAC_10GBOSADATA  {0} UNION all SELECT {4} FROM {2}MAC_f07959df8122  {0}
//     UNION all SELECT {4} FROM {2}MAC_00e04c686dd1  {0}  UNION all SELECT {4} FROM {2}MAC_00e04c3105fb  {0}
//     UNION all SELECT {4} FROM {2}MAC_70e400a2c0d4  {0}  UNION all SELECT {4} FROM {2}MAC_00262da5e778  {0}
//     UNION all SELECT {4} FROM {2}MAC_408d5cb2d04a  {0}  UNION all SELECT {4} FROM {2}MAC_408d5cb712a3  {0}
//     UNION all SELECT {4} FROM {2}MAC_408d5cb72176  {0}  UNION all SELECT {4} FROM {2}MAC_10634b007c4b  {0}
//     UNION all SELECT {4} FROM {2}MAC_10634b0966f0  {0}  UNION all SELECT {4} FROM {2}MAC_08626683922a  {0}
//     UNION all SELECT {4} FROM {2}MAC_c8e7d8e187a7  {0}  UNION all SELECT {4} FROM {2}MAC_f0b42937e924  {0}
//     UNION all SELECT {4} FROM {2}MAC_fcaa14afca45  {0}  UNION all SELECT {4} FROM {2}MAC_F07959DF968E  {0}
//     UNION all SELECT {4} FROM {2}MAC_f07959df9218  {0}  UNION all SELECT {4} FROM {2}MAC_f07959df9694  {0}
//     UNION all SELECT {4} FROM {2}MAC_f07959dfa742  {0}  UNION all SELECT {4} FROM {2}MAC_fcaa14db2983  {0}  ",
//     query_ty,"BOSAautotest_Data.dbo.","BOSAautotestDB.dbo.",testtype,testtype_12),&[&1i32]).await.unwrap();
//     let rowsets = stream.into_results().await.unwrap();
//     let mut rv: Vec<Vec<String>> = vec![];
//     for i in 0..rowsets.len() {
//         let rows = rowsets.get(i).unwrap();
//         for row in rows {
//             let mut v: Vec<String> = vec![];
//             v.push(row.get::<&str, _>(0).unwrap().to_string());
//             v.push(row.get::<&str, _>(1).unwrap().to_string());
//             v.push(row.get::<&str, _>(2).unwrap().to_string());
//             v.push(row.get::<&str, _>(3).unwrap().to_string());
//             v.push(row.get::<&str, _>(4).unwrap().to_string());
//             v.push(row.get::<&str, _>(5).unwrap().to_string());
//             v.push(row.get::<&str, _>(6).unwrap().to_string());
//             v.push(row.get::<&str, _>(7).unwrap().to_string());
//             v.push(row.get::<&str, _>(8).unwrap().to_string());
//             v.push(row.get::<&str, _>(9).unwrap().to_string());
//             v.push(row.get::<&str, _>(10).unwrap().to_string());
//             v.push(
//                 row.get::<chrono::NaiveDateTime, _>(11)
//                     .unwrap()
//                     .format("%Y/%m/%d %H:%M:%S")
//                     .to_string(),
//             );
//             rv.insert(i, v)
//         }
//     }
//     let sum = rv.len();
//     (rv, sum)
// }

// //####################################################################解绑############################################################################
// pub async fn bind_query_none() -> (Vec<Vec<String>>, usize) {
//     let mut client = client().await;
//     let select_all_info = "OLD_SN,NEW_SN,CREATOR,CREATE_TIME";
//     let stream  = client.query(format!("select {} from MES_SN_REPLACEMENT ORDER BY [CREATE_TIME] DESC OFFSET 0 ROWS FETCH NEXT 50 ROWS ONLY ",select_all_info),&[&1i32]).await.unwrap();
//     let rowsets = stream.into_results().await.unwrap();
//     let mut rv: Vec<Vec<String>> = vec![];
//     let no = rv.len();
//     for i in 0..rowsets.len() {
//         let rows = rowsets.get(i).unwrap();
//         for row in rows {
//             let mut v: Vec<String> = vec![];
//             v.push(row.get::<&str, _>(0).unwrap().to_string());
//             v.push(row.get::<&str, _>(1).unwrap().to_string());
//             v.push(row.get::<&str, _>(2).unwrap().to_string());
//             v.push(
//                 row.get::<chrono::NaiveDateTime, _>(3)
//                     .unwrap()
//                     .format("%Y/%m/%d %H:%M:%S")
//                     .to_string(),
//             );
//             rv.insert(i, v)
//         }
//     }
//     (rv, no)
// }

// pub async fn bind_today() -> usize {
//     let mut client = client().await;
//     let select_all_info = "OLD_SN";
//     let d1 = Local::now().format("%Y/%m/%d 00:00:00").to_string();
//     let d2 = Local::now().format("%Y/%m/%d 23:59:59").to_string();
//     let stream = client
//         .query(
//             format!(
//                 "select {} from MES_SN_REPLACEMENT where CREATE_TIME Between '{}' and '{}' ",
//                 select_all_info, d1, d2
//             ),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let rowsets = stream.into_results().await.unwrap();
//     let quantity = rowsets.len();
//     quantity
// }

// pub async fn init() -> Vec<Vec<String>> {
//     let mut client = client().await;
//     let select_all_info = "OLD_SN,NEW_SN,CREATOR,CREATE_TIME";
//     let stream  = client.query(format!("select {} from MES_SN_REPLACEMENT ORDER BY [CREATE_TIME] DESC OFFSET 0 ROWS FETCH NEXT 1 ROWS ONLY ",select_all_info),&[&1i32]).await.unwrap();
//     let row = stream.into_row().await.unwrap().unwrap();
//     let mut rv: Vec<Vec<String>> = vec![];
//     let mut v: Vec<String> = vec![];
//     v.push(row.get::<&str, _>(0).unwrap().to_string());
//     v.push(row.get::<&str, _>(1).unwrap().to_string());
//     v.push(row.get::<&str, _>(2).unwrap().to_string());
//     v.push(
//         row.get::<chrono::NaiveDateTime, _>(3)
//             .unwrap()
//             .format("%Y/%m/%d %H:%M:%S")
//             .to_string(),
//     );
//     rv.insert(0, v);

//     rv
// }

// pub async fn bind_query(s: String) -> (Vec<Vec<String>>, usize) {
//     let mut client = client().await;
//     let select_all_info = "OLD_SN,NEW_SN,CREATOR,CREATE_TIME";
//     let stream  = client.query(format!("select {0} from MES_SN_REPLACEMENT where OLD_SN = '{1}' or NEW_SN = '{1}' or OLD_SN = '{1}' or NEW_SN = '{1}'",select_all_info,s),&[&1i32]).await.unwrap();
//     let rowsets = stream.into_results().await.unwrap();
//     let mut rv: Vec<Vec<String>> = vec![];
//     let no = rv.len();
//     for i in 0..rowsets.len() {
//         let rows = rowsets.get(i).unwrap();
//         for row in rows {
//             let mut v: Vec<String> = vec![];
//             v.push(row.get::<&str, _>(0).unwrap().to_string());
//             v.push(row.get::<&str, _>(1).unwrap().to_string());
//             v.push(row.get::<&str, _>(2).unwrap().to_string());
//             v.push(
//                 row.get::<chrono::NaiveDateTime, _>(3)
//                     .unwrap()
//                     .format("%Y/%m/%d %H:%M:%S")
//                     .to_string(),
//             );
//             rv.insert(i, v)
//         }
//     }
//     (rv, no)
// }

// pub async fn check_pn(b: String, w: String) -> ((Vec<Vec<String>>, usize), Tip) {
//     let mut client = client().await;
//     let mut client_1 = client_1().await;
//     let joborder1 = &b[2..8];
//     let joborder2 = &w[2..8];
//     let stream = client
//         .query(
//             format!(
//                 "select PN from Materiel_Joborder where Joborder = '{}' ",
//                 joborder1
//             ),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let stream_1 = client_1
//         .query(
//             format!(
//                 "select PN from Materiel_Joborder where Joborder = '{}' ",
//                 joborder2
//             ),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let rowsets = stream.into_results().await.unwrap();
//     let row = rowsets.get(0).unwrap();
//     let j1 = row.get(0).unwrap();
//     let job_1 = j1.get::<&str, _>(0).unwrap().to_string();
//     let rowsets_1 = stream_1.into_results().await.unwrap();
//     let rows_1 = rowsets_1.get(0).unwrap();
//     let j2 = rows_1.get(0).unwrap();
//     let job_2 = j2.get::<&str, _>(0).unwrap().to_string();
//     let x = init().await;
//     let y = bind_today().await;
//     if job_1 == job_2 {
//         return ((x, y), Tip::Hl);
//     }
//     let w_sn = w.clone();
//     let b_sn = b.clone();
//     let t = check_status(w_sn, b_sn).await;
//     return t;
// }

// pub async fn check_status(b: String, w: String) -> ((Vec<Vec<String>>, usize), Tip) {
//     let mut client = client().await;
//     let w_sn = w.clone();
//     let stream = client
//         .query(
//             format!("select * from Barcode_status where sn = '{}'", w_sn),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let rowsets = stream.into_results().await.unwrap();
//     let x = init().await;
//     let y = bind_today().await;
//     for i in rowsets {
//         if i.is_empty() {
//             return ((x, y), Tip::Add);
//         }
//     }

//     let w_sn = w.clone();
//     let b_sn = b.clone();
//     let t = check_bind(b_sn, w_sn).await;
//     return t;
// }
// pub async fn check_bind(b: String, w: String) -> ((Vec<Vec<String>>, usize), Tip) {
//     let mut client = client().await;
//     let b_sn = b.clone();
//     let w_sn = w.clone();
//     let stream = client
//         .query(
//             format!(
//                 "select * from QA_snRelation where SN = '{}' or SN_ShipMent = '{}'",
//                 b_sn, w_sn
//             ),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let rowsets = stream.into_results().await.unwrap();
//     let x = init().await;
//     let y = bind_today().await;
//     for i in rowsets {
//         if i.len() != 0 {
//             return ((x, y), Tip::Binded);
//         }
//     }
//     let b_sn = b.clone();
//     let w_sn = w.clone();
//     let t = insert(b_sn, w_sn).await;
//     return t;
// }
// pub async fn insert_test() {
//     let mut client = client().await;
//     let insert_1 = "(Pack_no,Sn,PN,WorkOrder,Creator,CreateTime,Id)";
//     let boxsn = "H012345678910";
//     let sn = "HD0123456789101231";
//     let pn = "30300085";
//     let wo = "233345";
//     let relationtime = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
//     let user_no = "Y1002";
//     let mut id = 10000;
//     // let id = Uuid::new_v4().to_string().to_uppercase();
//     for x in 0..=99999{
//         println!("id = {}",id);
//     let _stream = client
//         .execute(
//             format!(
//                 "INSERT into MaterialPackSn {0} values ('{1}','{2}','{3}','{4}','{5}','{6}','{7}')",insert_1,boxsn,sn,pn,wo,user_no,relationtime,id),&[&1i32]
//         )
//         .await
//         .unwrap();
//         id += 1
//     }
// }
// pub async fn insert(b_sn: String, w_sn: String) -> ((Vec<Vec<String>>, usize), Tip) {
//     let mut client = client().await;
//     let insert_1 = "(SN_ShipMent,SN,BoxNum,JobOrder,Status,Relationtime,userno,GUID)";
//     let job = w_sn.clone();
//     let re_sn_ship_ment = b_sn.clone();
//     let re_sn = w_sn.clone();
//     let re_job_order = &job[2..8];
//     let re_box_num = "0";
//     let status = "0";
//     let relationtime = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
//     let user_no = "Y1002";
//     let id = Uuid::new_v4().to_string().to_uppercase();
//     let _stream = client
//         .execute(
//             format!(
//                 "INSERT into QA_snRelation {0} values ('{1}','{2}','{3}','{4}','{5}','{6}','{7}','{8}')",insert_1,re_sn_ship_ment,re_sn,re_box_num,re_job_order,status,relationtime,user_no,id),&[&1i32]
//         )
//         .await
//         .unwrap();
//     let x = init().await;
//     let y = bind_today().await;
//     let wx = w_sn.clone();
//     let bd = b_sn.clone();
//     insert_2(bd, wx).await;
//     let t = Tip::Ok;
//     return ((x, y), t);
// }
// pub async fn insert_2(b_sn: String, w_sn: String) {
//     let mut client = client().await;
//     let insert_1 = "(OLD_SN,NEW_SN,RelationType,BARCODE_TYPE,OPERATOR,CREATOR,CREATE_TIME,SN_FROM)";
//     let re_sn_ship_ment = b_sn.clone();
//     let re_sn = w_sn.clone();
//     let sn_from = "From_Cust";
//     let relation_type = "SN_LINK";
//     let barcode_type = "SSN";
//     let relationtime = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
//     let user_no = "Y1002";
//     let _stream = client
//         .execute(
//             format!(
//                 "INSERT into MES_SN_REPLACEMENT {0} values ('{1}','{2}','{3}','{4}','{5}','{5}','{6}','{7}')",
//                 insert_1,re_sn,re_sn_ship_ment,relation_type,barcode_type,user_no,relationtime,sn_from),&[&1i32]
//         )
//         .await
//         .unwrap();
// }
// pub async fn check_cz(id: String, pa: String) -> Rec {
//     let mut client = qx_client().await;
//     let i = id.clone();
//     let stream = client
//         .query(
//             format!("select worker_id from login where worker_id = '{}'", i),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let row = stream.into_results().await.unwrap();
//     let r = row.get(0).unwrap();
//     if r.is_empty() {
//         return Rec::Bcz;
//     }
//     let p_id = id.clone();
//     let p_pa = pa.clone();
//     let p = check_permission(p_id, p_pa).await;
//     return p;
// }

// pub async fn check_permission(id: String, pa: String) -> Rec {
//     let mut client = qx_client().await;
//     let i = id.clone();
//     let stream = client
//         .query(
//             format!("select permission from login where worker_id = '{}'", i),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let row = stream.into_results().await.unwrap();
//     let r = row.get(0).unwrap();
//     for o in r {
//         let perm = o.get::<&str, _>(0).unwrap().to_string();
//         if perm != "0".to_string() {
//             return Rec::QxErr;
//         }
//     }
//     let p_id = id.clone();
//     let p_pa = pa.clone();
//     let pa = check_pass(p_id, p_pa).await;
//     return pa;
// }

// pub async fn check_pass(id: String, pa: String) -> Rec {
//     let mut client = qx_client().await;
//     let i = id.clone();
//     let p = pa.clone();
//     let stream = client
//         .query(
//             format!("select unbing_pass from login where worker_id = '{}'", i),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let row = stream.into_row().await.unwrap().unwrap();
//     let perm = row.get::<&str, _>(0).unwrap().to_string();
//     if perm != p {
//         return Rec::PassErr;
//     }

//     return Rec::UnbindOk;
// }

// pub async fn unbind_0(s: String, i: String) {
//     let mut client = client().await;
//     let mut client_1 = client_1().await;
//     let select = "SN_ShipMent,SN,BoxNum,JobOrder,Status,Relationtime,userno,ID,GUID";
//     let re_sn = s.clone();
//     let stream = client
//         .query(
//             format!(
//                 "select {0} from QA_snRelation where SN = '{1}' or SN_ShipMent = '{1}'",
//                 select, re_sn
//             ),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let row = stream.into_row().await.unwrap().unwrap();
//     let sn_ship_menttt = row.get::<&str, _>(0).unwrap().to_string();
//     let sn = row.get::<&str, _>(1).unwrap().to_string();
//     let box_num = row.get::<&str, _>(2).unwrap().to_string();
//     let job_order = row.get::<&str, _>(3).unwrap().to_string();
//     let status = row.get::<&str, _>(4).unwrap().to_string();
//     let relationtime = row
//         .get::<chrono::NaiveDateTime, _>(5)
//         .unwrap()
//         .format("%Y/%m/%d %H:%M:%S")
//         .to_string();
//     let userno = row.get::<&str, _>(6).unwrap().to_string();
//     let id = row.get::<&str, _>(7).unwrap().to_string();
//     let guid = row.get::<&str, _>(8).unwrap().to_string();
//     let date = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
//     let unbind_id = i.clone();
//     let insert =
//         "(SN_ShipMent,SN,BoxNum,JobOrder,Status,Relationtime,userno,ID,GUID,Creator,CreateDate)";
//     let _stream = client_1
//         .execute(
//             format!(
//                 "INSERT into QA_snRelation_history {} values ('{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}')",
//                 insert,sn_ship_menttt,sn,box_num,job_order,status,relationtime,userno,id,guid,unbind_id,date),&[&1i32]
//         )
//         .await
//         .unwrap();

//     // let x = init().await;
//     // let y = bind_today().await;
//     // return (x,y);
// }

// pub async fn unbind_2(wx_sn: String, i: String) -> Rec {
//     let mut client = client().await;
//     let re_sn = wx_sn.clone();
//     let _stream = client
//         .execute(
//             format!("DELETE FROM MES_SN_REPLACEMENT WHERE OLD_SN ='{}'", re_sn),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let s_0 = wx_sn.clone();
//     let s_1 = wx_sn.clone();
//     let i_1 = i.clone();
//     unbind_0(s_0, i_1).await;
//     unbind_3(s_1).await;
//     return Rec::UnbindOk;
// }

// pub async fn unbind_3(wx_sn: String) {
//     let mut client = client().await;
//     let re_sn = wx_sn.clone();
//     let _stream = client
//         .execute(
//             format!("DELETE FROM QA_snRelation WHERE SN ='{}'", re_sn),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
// }

// //####################################################################工单############################################################################
// pub async fn query_job(s: String) -> (Vec<String>, String) {
//     let mut client = client().await;

//     let order = "joborder,PN,ProdType,Quantity";
//     let stream = client
//         .query(
//             format!(
//                 "select {0} from Materiel_Joborder where joborder = '{1}'",
//                 order, s
//             ),
//             &[&1i32],
//         )
//         .await
//         .unwrap();

//     let row = stream.into_row().await.unwrap().unwrap();
//     let mut str = String::from("工单号：");
//     let job_no = row.get::<&str, _>(0).unwrap();
//     let pn = row.get::<&str, _>(1).unwrap();
//     let prodtype = row.get::<&str, _>(2).unwrap();
//     let quantity = row.get::<i32, _>(3).unwrap();
//     let q = quantity.to_string();
//     let q_s = q.as_str();
//     str.push_str(job_no);
//     str.push_str(" . 料号：");
//     str.push_str(pn);
//     str.push_str(" . 型号：");
//     str.push_str(prodtype);
//     str.push_str(" . 工单数量：");
//     str.push_str(q_s);
//     let mut pn_infos = pn[4..].to_string();
//     let x = job_no.clone();
//     pn_infos.push_str(x);
//     let q_i32 = quantity.clone();
//     let st = query_job_id(pn_infos.as_str(), q_i32, str, pn, job_no).await;
//     st
// }
// pub async fn query_job_id(
//     s: &str,
//     q: i32,
//     str: String,
//     pn: &str,
//     job_no: &str,
// ) -> (Vec<String>, String) {
//     let mut client_1 = client_1().await;
//     let stream = client_1
//         .query(
//             format!(
//                 "select CURRENT_MAXID from SN_POLICY_MAX where NONE_ID_PART = '{}'",
//                 s
//             ),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let row = stream.into_results().await.unwrap();
//     let mut str_1 = str.clone();
//     let mut v = vec![];
//     for i in row.iter() {
//         if i.is_empty() {
//             let m_i32 = 0;
//             let m = m_i32.clone();
//             let m_1 = m_i32.clone().to_string();
//             let s = q - m_i32;
//             let s_1 = s.clone();
//             let s_2 = s.clone().to_string();
//             str_1.push_str(" . 已生成数量：");
//             str_1.push_str(m.to_string().as_str());
//             str_1.push_str(" . 剩余数量：");
//             str_1.push_str((s_1.to_string()).as_str());
//             v.push(m_1);
//             v.push(s_2);
//             v.push(pn.to_string());
//             v.push(job_no.to_string());
//         } else {
//             for x in i {
//                 let much = x.get::<&str, _>(0).unwrap();
//                 let m_i32 = much.parse::<i32>().unwrap();
//                 let m_1 = much.clone().to_string();
//                 let s = q - m_i32;
//                 let s_2 = s.clone().to_string();
//                 str_1.push_str(" . 已生成数量：");
//                 str_1.push_str(much);
//                 str_1.push_str(" . 剩余数量：");
//                 str_1.push_str((s.to_string()).as_str());
//                 v.push(m_1);
//                 v.push(s_2);
//                 v.push(pn.to_string());
//                 v.push(job_no.to_string());
//             }
//         }
//     }
//     (v, str_1)
// }

// pub async fn query_rule_id(infos: Vec<String>, mcu: i32) -> Vec<Vec<String>> {
//     let info = infos.clone();
//     let pn = info.get(2).unwrap();
//     let m = info.get(0).unwrap().parse::<i32>().unwrap();
//     let mut client = client().await;
//     let stream = client
//         .query(
//             format!("select Ruleid from SNCreate_PN where PN = '{}'", pn),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let row = stream.into_row().await.unwrap().unwrap();
//     let rule_id = row.get::<i32, _>(0).unwrap().to_string();
//     let mut client_1 = qx_client().await;
//     let rule_query = "StartStr,MiddleStr,EndStr,Digit,orderLen";
//     let stream = client_1
//         .query(
//             format!(
//                 "select {} from SN_Rule where Ruleid = '{}'",
//                 rule_query, rule_id
//             ),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let row = stream.into_row().await.unwrap().unwrap();
//     let start = match row.get::<&str, _>(0) {
//         Some(start) => start,
//         None => "",
//     };
//     let mid = match row.get::<&str, _>(1) {
//         Some(mid) => mid,
//         None => "",
//     };
//     let end = match row.get::<&str, _>(2) {
//         Some(end) => end,
//         None => "",
//     };
//     let sn_id = match row.get::<i32, _>(3) {
//         Some(6) => format!("{:06}", m + 1),
//         Some(5) => format!("{:05}", m + 1),
//         Some(4) => format!("{:04}", m + 1),
//         _ => format!("{:06}", m + 1),
//     };
//     let order_len = row.get::<i32, _>(4).unwrap();
//     let job_order = info.get(3).unwrap();
//     let job_order = match order_len {
//         6 => &job_order[..],
//         5 => &job_order[1..],
//         4 => &job_order[2..],
//         _ => &job_order[..],
//     };

//     let add_sn = add_sn(start, mid, end, job_order, pn, sn_id, mcu, rule_id).await;
//     add_sn
// }

// pub async fn add_sn(
//     start: &str,
//     mid: &str,
//     end: &str,
//     job_order: &str,
//     pn: &str,
//     sn_id: String,
//     muc: i32,
//     rule_id: String,
// ) -> Vec<Vec<String>> {
//     let aa_sn = sn_id.clone().parse::<i32>().unwrap();
//     let o_sn = sn_id.clone().parse::<i32>().unwrap() + muc - 1;
//     let pn_s = &pn[4..];
//     let mut rv = vec![];
//     let mut sn_v = vec![];
//     let mut x = 0;
//     let mut y = 0;
//     //let mut out_sn = vec![];
//     for i in aa_sn..=o_sn {
//         x += 1;
//         let job_order_2 = job_order.clone();
//         let mut v = vec![];
//         let mut sn = start.to_string();
//         let i = format!("{:06}", i);
//         sn.push_str(pn_s);
//         sn.push_str(job_order_2);
//         sn.push_str(mid);
//         sn.push_str(i.to_string().as_str());
//         sn.push_str(end);
//         let v_sv = sn.clone();
//         v.push(v_sv);
//         v.push(x.to_string());
//         v.push(job_order.to_string());
//         v.push(pn.to_owned());
//         let sn_vv = sn.clone();
//         sn_v.insert(y, sn_vv);
//         rv.insert(y, v);
//         y += 1;
//     }
//     let sn_vss = sn_v.clone();
//     let job_order_1 = job_order.clone();
//     let pn_1 = pn.clone();
//     insert_sn(sn_vss, job_order_1, pn_1, rule_id, o_sn).await;
//     rv
// }
// pub async fn insert_sn(sn_v: Vec<String>, job_order: &str, pn: &str, rule_id: String, o_sn: i32) {
//     let mut client = client().await;
//     let joborder = job_order.clone();
//     let prodorder_name = "物料组";
//     let sn_type = "SSN";
//     let uuid = Uuid::new_v4().to_string().to_uppercase();
//     let createtime = Local::now().to_string();
//     let strdate = &createtime.as_str()[..23];
//     let sql = "(joborder,sn,prodorder_name,SN_type,CreateTime,GUID)";
//     for sn in &sn_v {
//         let _stream = client
//             .execute(
//                 format!(
//                     "INSERT into Barcode_jobsn {0} values
//         ('{1}','{2}','{3}','{4}','{5}','{6}')",
//                     sql, joborder, sn, prodorder_name, sn_type, strdate, uuid
//                 ),
//                 &[&1i32],
//             )
//             .await
//             .unwrap();
//     }
//     let sn_1 = sn_v.clone();
//     let pn_1 = pn.clone();
//     insert_sn_1(sn_1, job_order, pn_1, rule_id, o_sn).await;
// }
// pub async fn insert_sn_1(sn_v: Vec<String>, job_order: &str, pn: &str, rule_id: String, o_sn: i32) {
//     let mut client = client().await;
//     let joborder = job_order.clone();
//     let prodorder_name = "物料组";
//     let sn_type = "SSN";
//     let is_mrb = "N";
//     let qty = "1";
//     let under_work = "0";
//     let used = "0";
//     let done_pn_option_id = "-1";
//     let doing_pn_option_id = "-1";
//     let next_pn_option_id = "-1";
//     let status = "-2";
//     let now = "-1";
//     let guid_2 = Uuid::new_v4().to_string().to_uppercase();
//     let createtime = Local::now().format("%F %T").to_string();
//     let date = Local::now().to_string();
//     let strdate = &date.as_str()[..23];
//     let sn_pn = pn.clone();
//     let sql = "(joborder,sn,prodorder_name,IsMrb,Transfertime,UnderWork,used,DonePnOption_id,DoingPnOption_id,NextPnOption_id,Status,now,InTime,GUID,PN,QTY,SN_TYPE)";
//     for sn in sn_v {
//         let _stream  = client.execute(format!("INSERT into Barcode_status {0} values
//         ('{1}','{2}','{3}','{4}','{5}','{6}','{7}','{8}','{9}','{10}','{11}','{12}','{13}','{14}','{15}','{16}','{17}')",
//         sql,joborder,sn,prodorder_name,is_mrb,createtime,under_work,used,done_pn_option_id,doing_pn_option_id,next_pn_option_id,status,now,strdate,guid_2,sn_pn,qty,sn_type
//     ),&[&1i32]).await.unwrap();
//     }
//     select_max_sn(joborder, sn_pn, rule_id, o_sn).await;
// }

// pub async fn select_max_sn(job_order: &str, sn_pn: &str, rule_id: String, o_sn: i32) {
//     let mut client_1 = client_1().await;
//     let mut pn = (&sn_pn.clone()[4..]).to_string();
//     pn.push_str(job_order);
//     let stream = client_1
//         .query(
//             format!(
//                 "select CURRENT_MAXID from SN_POLICY_MAX where NONE_ID_PART = '{}'",
//                 pn
//             ),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let row = stream.into_results().await.unwrap();
//     for i in row.iter() {
//         if i.is_empty() {
//             let mut client = client().await;
//             let fac = "HYD01";
//             let si = "HZ";
//             let seg = "1";
//             let sn_ty = "SSN";
//             let uuid = Uuid::new_v4().to_string().to_uppercase();
//             let max_sn = "(FACILITY,SITE,NONE_ID_PART,CURRENT_MAXID,POLICY_ID,SEG_NO,SN_TYPE,NONE_ID_PART_FULL,GUID)";
//             let _stream  = client.execute(format!("INSERT into SN_POLICY_MAX {0} values ('{1}','{2}','{3}','{4}','{5}','{6}','{7}','{8}','{9}')",
//             max_sn,fac,si,pn,o_sn,rule_id,seg,sn_ty,pn,uuid),&[&1i32]).await.unwrap();
//         } else {
//             let mut client_2 = client_2().await;
//             let _stream = client_2
//                 .execute(
//                     format!(
//                         "UPDATE SN_POLICY_MAX set CURRENT_MAXID = '{}' WHERE NONE_ID_PART = '{}'",
//                         o_sn, pn
//                     ),
//                     &[&1i32],
//                 )
//                 .await
//                 .unwrap();
//         }
//     }
// }

// pub async fn login_user(id: String,pa: String) -> bool {
//     let mut client = qx_client().await;
//     let i = id.clone();
//     let stream = client
//         .query(
//             format!("select worker_id from login where worker_id = '{}'", i),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let row = stream.into_results().await.unwrap();
//     let r = row.get(0).unwrap();
//     if r.is_empty() {
//         return false;
//     } else {
//         let x = login_pass(id,pa).await;
//         return x;
//     }
// }

// pub async fn login_pass(id: String, p: String) -> bool {
//     let mut client = qx_client().await;
//     let i = id.clone();
//     let stream = client
//         .query(
//             format!("select password from login where worker_id = '{}'", i),
//             &[&1i32],
//         )
//         .await
//         .unwrap();
//     let row = stream.into_row().await.unwrap().unwrap();
//     let r = row.get::<&str, _>(0).unwrap();

//     if r != p {
//         return false;
//     } else {
//         return true;
//     }
// }
