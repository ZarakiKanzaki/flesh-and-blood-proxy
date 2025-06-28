if [ ! -d "./tmp" ]; then
    mkdir ./tmp;
fi

if [ ! -d "./tmp/flesh-and-blood-cards" ]; then
    cd ./tmp;
    git clone https://github.com/the-fab-cube/flesh-and-blood-cards.git;
    cd ..
fi 

cd  ./tmp/flesh-and-blood-cards;
updated_branch=`echo "$(git branch -r --sort=-committerdate)" | head -1`
updated_branch=""$(printf "%s" "$updated_branch" | cut -d/ -f2-)"" 

git checkout $updated_branchgit
cp -r ./json ../../db/;
