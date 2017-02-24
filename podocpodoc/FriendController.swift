//
//  FriendController.swift
//  podocpodoc
//
//  Created by soonhyung-imac on 2/20/17.
//  Copyright © 2017 twentyhours. All rights reserved.
//

import UIKit

import RxSwift
import RxDataSources

class FriendController: UIViewController {

    let dataSource = RxTableViewSectionedReloadDataSource<SectionModel<String, Friend>>()
    let viewModel = FriendViewModel()
    let disposeBag = DisposeBag()
    
    @IBOutlet weak var tableView: UITableView!
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        self.navigationItem.title = "Friends"
        
        tableView.register(UITableViewCell.self, forCellReuseIdentifier: "Cell")

        // Do any additional setup after loading the view.
        dataSource.configureCell = { (_, tv, ip, item) in
            let cell = tv.dequeueReusableCell(withIdentifier: "Cell", for: ip)
            cell.textLabel?.text = "Friend: \(item.label), \(item.address)"
            return cell
        }
        
        viewModel.getUsers()
            .bindTo(tableView.rx.items(dataSource: dataSource))
            .addDisposableTo(disposeBag)
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
    

    /*
    // MARK: - Navigation

    // In a storyboard-based application, you will often want to do a little preparation before navigation
    override func prepare(for segue: UIStoryboardSegue, sender: Any?) {
        // Get the new view controller using segue.destinationViewController.
        // Pass the selected object to the new view controller.
    }
    */

}
